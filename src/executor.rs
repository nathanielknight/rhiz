use crate::ast;
use crate::compiler;
use crate::functions;

pub type ExecutionError = Box<std::error::Error>;
pub type ExecutionResult = Result<(), ExecutionError>;

pub trait Executable {
    fn run(&self) -> ExecutionResult;
}

pub fn execute(func_name: &ast::RhizValue, args: &[ast::RhizValue]) -> ExecutionResult {
    let func = match functions::look_up_function(&func_name) {
        Some(f) => f,
        None => {
            let msg = format!("Invalid function: {:?}", func_name);
            return Err(ExecutionError::from(msg));
        }
    };
    func(args)
}

fn exec_sexpr(contents: &[ast::RhizValue]) -> ExecutionResult {
    if contents.len() == 0 {
        let msg = "Can't eval an empty expression";
        return Err(ExecutionError::from(msg));
    }
    let name = &contents[0];
    let args = &contents[1..contents.len()];
    execute(name, args)
}

pub fn exec_task(task_name: &str, program: compiler::CompiledProgram) -> ExecutionResult {
    let task = match program.tasks.get(task_name) {
        Some(t) => t,
        None => {
            return Err(ExecutionError::from(format!(
                "No such task: '{}'",
                task_name
            )))
        }
    };
    for item in &task.items {
        match item {
            ast::RhizValue::SExpr(contents) => exec_sexpr(contents)?,
            _ => unreachable!(),
        };
    }
    Ok(())
}
