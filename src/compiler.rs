use std::collections::HashMap;

use crate::ast;

pub type CompilationError = Box<dyn std::error::Error>;
pub type CompilationResult<T> = Result<T, CompilationError>;

/// Compilation target for s-xpressions of the format
/// ```ignore
/// (task "name" ["description"] [funcall]*)
/// ```
pub struct Task<'a> {
    pub name: String,
    pub description: Option<String>,
    pub items: Vec<&'a ast::RhizValue>,
}

impl<'a> Task<'a> {
    fn compile(sexpr: &'a ast::RhizValue) -> CompilationResult<Task<'a>> {
        let items = match sexpr {
            ast::RhizValue::SExpr(items) => items,
            _ => return Err(CompilationError::from("Expected a sexpr to make a task")),
        };
        if items.len() < 2 {
            return Err(CompilationError::from("Invalid task declaration"));
        };
        match &items[0] {
            ast::RhizValue::Symbol(s) => {
                if s != "task" {
                    let msg = "Only 'task' declarations allowed at the top-level of a Rhizfile";
                    return Err(CompilationError::from(msg));
                }
            }
            _ => {
                let msg = "Top-level Rhizfile declarations should be of the form (task name [description] [commands]*)";
                return Err(CompilationError::from(msg));
            }
        }
        let name = match &items[1] {
            ast::RhizValue::String(s) => s.to_owned(),
            _ => {
                let msg = "Task names should be strings";
                return Err(CompilationError::from(msg));
            }
        };
        let description = if items.len() > 2 {
            match &items[2] {
                ast::RhizValue::String(s) => Some(s.to_owned()),
                _ => None,
            }
        } else {
            None
        };
        let rest = match description {
            Some(_) => &items[3..],
            None => &items[2..],
        };
        if rest.iter().any(|v| !matches!(v, ast::RhizValue::SExpr(_))) {
            let msg = "Tasks should only contain SExprs";
            return Err(CompilationError::from(msg));
        }
        Ok(Task {
            name,
            description,
            items: rest.iter().collect(),
        })
    }
}

pub fn compile<'a>(prog: &'a ast::RhizValue) -> CompilationResult<HashMap<String, Task<'a>>> {
    match prog {
        ast::RhizValue::Program(tasks) => {
            let compiled_tasks = tasks.iter().map(Task::compile);
            let mut tasks: HashMap<String, Task<'a>> = HashMap::new();
            for task in compiled_tasks {
                match task {
                    Ok(t) => tasks.insert(t.name.to_owned(), t),
                    Err(e) => return Err(e),
                };
            }
            Ok(tasks)
        }
        _ => Err(CompilationError::from(
            "I only know how to compile programs",
        )),
    }
}
