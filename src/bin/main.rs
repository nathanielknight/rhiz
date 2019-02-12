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

fn file_dir<'a>(filepath: &'a std::path::PathBuf) -> CommandResult<&'a std::path::Path> {
    filepath
        .parent()
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
        println!(" {: <12} :  {}", name, desc);
    }
    Ok(())
}

fn main() -> CommandResult<()> {
    use std::env;

    let rhizfile_path = &find_rhizfile()?;
    let working_dir_path = file_dir(rhizfile_path)?;

    let src = std::fs::read_to_string(rhizfile_path)?;
    let parsed = &ast::parse_rhiz_program(&src)?;
    let tasks = &compiler::compile(parsed)?;

    match env::args().nth(1) {
        Some(tname) => executor::exec_task(&tname, tasks, working_dir_path),
        None => print_tasks(rhizfile_path, tasks),
    }
}
