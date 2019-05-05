use std::collections::HashMap;
use std::path::Path;

use crate::ast;
use crate::compiler;
use crate::functions;

pub type ExecutionResult = Result<(), ExecutionError>;

#[derive(Debug)]
pub struct ExecutionError {
    msg: String,
}

impl From<String> for ExecutionError {
    fn from(msg: String) -> Self {
        ExecutionError { msg }
    }
}

impl From<&str> for ExecutionError {
    fn from(msg: &str) -> Self {
        ExecutionError {
            msg: msg.to_owned(),
        }
    }
}

impl From<Box<std::error::Error>> for ExecutionError {
    fn from(error: Box<std::error::Error>) -> Self {
        ExecutionError {
            msg: error.to_string(),
        }
    }
}

impl From<std::io::Error> for ExecutionError {
    fn from(error: std::io::Error) -> Self {
        ExecutionError {
            msg: error.to_string(),
        }
    }
}

pub fn execute(
    func_name: &ast::RhizValue,
    args: &[ast::RhizValue],
    working_dir: &Path,
) -> ExecutionResult {
    let func = match functions::look_up_function(&func_name) {
        Some(f) => f,
        None => {
            let msg = format!("Invalid function: {:?}", func_name);
            return Err(ExecutionError::from(msg));
        }
    };
    func(args, working_dir)
}

pub fn exec_sexpr(contents: &[ast::RhizValue], working_dir: &Path) -> ExecutionResult {
    if contents.is_empty() {
        let msg = "Can't eval an empty expression";
        return Err(ExecutionError::from(msg));
    }
    let name = &contents[0];
    let args = &contents[1..contents.len()];
    execute(name, args, working_dir)
}

pub fn exec_task<S>(
    task_name: &str,
    tasks: &HashMap<String, compiler::Task, S>,
    working_dir: &Path,
) -> ExecutionResult
where
    S: ::std::hash::BuildHasher,
{
    let task = match tasks.get(task_name) {
        Some(t) => t,
        None => {
            return Err(ExecutionError::from(format!(
                "No such task: '{}'",
                task_name
            )));
        }
    };
    for item in &task.items {
        match item {
            ast::RhizValue::SExpr(contents) => exec_sexpr(contents, working_dir)?,
            _ => unreachable!(),
        };
    }
    Ok(())
}
