type CommandError = Box<std::error::Error>;
type CommandResult<T> = Result<T, CommandError>;

const RHIZFILE_PATTERN: &str = "[Rr]hizfile";

fn rhizfile_in_dir(dirpath: &std::path::Path) -> Option<std::path::PathBuf> {
    use glob;
    let pattern_src = format!(
        "{}/{}",
        dirpath.to_str().expect("paths should be valid utf-8"),
        RHIZFILE_PATTERN
    );
    let mut paths = glob::glob(&pattern_src).ok()?;
    let path = paths.next()?.ok()?;
    Some(path)
}

fn find_rhizfile_dir() -> CommandResult<std::path::PathBuf> {
    use std::env;
    let exec_dir = env::current_dir()?;
    let mut work_dir = exec_dir.as_path();
    loop {
        match rhizfile_in_dir(work_dir) {
            Some(rhizfile_path) => {
                return rhizfile_path
                    .parent()
                    .map(|p| p.to_owned())
                    .ok_or_else(|| CommandError::from("Rhizfile doesn't have a parent?"));
            }
            None => match work_dir.parent() {
                Some(p) => {
                    work_dir = p;
                    continue;
                }
                None => return Err(CommandError::from("No Rhizfile found")),
            },
        }
    }
}

fn main() -> CommandResult<()> {
    let rhiz_dir = find_rhizfile_dir()?;
    println!("{:?}", rhiz_dir);
    Ok(())
}
