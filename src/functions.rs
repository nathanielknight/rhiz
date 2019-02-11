use crate::ast::RhizValue;
use crate::executor::{ExecutionError, ExecutionResult};

type RhizFunction = Fn(&[RhizValue]) -> ExecutionResult;

pub fn look_up_function(func_name: &RhizValue) -> Option<Box<RhizFunction>> {
    let symbol_name = match func_name {
        RhizValue::Symbol(s) => s,
        _ => return None,
    };
    match symbol_name.as_ref() {
        "log" => Some(Box::new(log)),
        "abort" => Some(Box::new(abort)),
        _ => None,
    }
}

fn log(args: &[RhizValue]) -> ExecutionResult {
    if args.len() != 1 {
        return Err(ExecutionError::from("`log` takes one argument"));
    }
    match &args[0] {
        RhizValue::String(s) => {
            println!("{}", s);
            Ok(())
        }
        _ => Err(ExecutionError::from("`log` takes a string")),
    }
}

fn abort(args: &[RhizValue]) -> ExecutionResult {
    if args.len() != 1 {
        return Err(ExecutionError::from("`abort` takes one argument"));
    }
    match &args[0] {
        RhizValue::String(s) => {
            eprintln!("{}", s);
            Err(ExecutionError::from("Execution aborted"))
        }
        _ => Err(ExecutionError::from("`abort` takes a string")),
    }
}
