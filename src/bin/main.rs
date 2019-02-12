type CommandError = Box<std::error::Error>;
type CommandResult<T> = Result<T, CommandError>;

use rhiz::ast;
use rhiz::compiler;
use rhiz::executor;

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

fn find_rhizfile() -> CommandResult<std::path::PathBuf> {
    use std::env;
    let exec_dir = env::current_dir()?;
    let mut work_dir = exec_dir.as_path();
    loop {
        match rhizfile_in_dir(work_dir) {
            Some(rhizfile_path) => return Ok(rhizfile_path),
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

fn file_dir(filepath: &std::path::PathBuf) -> CommandResult<std::path::PathBuf> {
    filepath
        .parent()
        .map(|p| p.to_owned())
        .ok_or(CommandError::from("Rhizfile has no parent?"))
}

fn print_tasks(
    path: &std::path::PathBuf,
    tasks: &std::collections::HashMap<String, compiler::Task>,
) -> CommandResult<()> {
    println!("Tasks in '{}': ", path.display());
    for (name, task) in tasks.iter() {
        let desc = match &task.description {
            Some(t) => t,
            None => "",
        };
        println!("  {} : {}", name, desc);
    }
    Ok(())
}

fn main() -> CommandResult<()> {
    use std::env;

    let rhiz_path_buf = &find_rhizfile()?;
    let rhiz_path = rhiz_path_buf.as_path();

    let src = std::fs::read_to_string(rhiz_path_buf)?;
    let parsed = &ast::parse_rhiz_program(&src)?;
    let tasks = &compiler::compile(parsed)?;

    match env::args().nth(1) {
        Some(tname) => executor::exec_task(&tname, tasks, rhiz_path),
        None => print_tasks(rhiz_path_buf, tasks),
    }
}
