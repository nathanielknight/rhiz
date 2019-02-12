use crate::ast::RhizValue;
use crate::executor::{ExecutionError, ExecutionResult};

type RhizFunction = Fn(&[RhizValue]) -> ExecutionResult;

macro_rules! error_with {
    ($msg:expr) => {
        return Err(ExecutionError::from($msg));
    };
}

macro_rules! check_args_len {
    ( $fname:expr, $args:expr, $cnt:expr ) => {
        if $args.len() != $cnt {
            let msg = format!("`{}` takes {} argument(s)", $fname, $cnt);
            error_with!(msg)
        }
    };
}

pub fn look_up_function(func_name: &RhizValue) -> Option<Box<RhizFunction>> {
    let symbol_name = match func_name {
        RhizValue::Symbol(s) => s,
        _ => return None,
    };
    match symbol_name.as_ref() {
        "log" => Some(Box::new(log)),
        "abort" => Some(Box::new(abort)),
        "delete-file" => Some(Box::new(delete_file)),
        _ => None,
    }
}

fn log(args: &[RhizValue]) -> ExecutionResult {
    check_args_len!("log", args, 1);
    let msg = if let RhizValue::String(s) = &args[0] {
        s
    } else {
        error_with!("`log` takes a string");
    };
    println!("{}", msg);
    Ok(())
}

fn abort(args: &[RhizValue]) -> ExecutionResult {
    check_args_len!("abort", args, 1);
    let msg = if let RhizValue::String(s) = &args[0] {
        s
    } else {
        error_with!("`abort` takes a string");
    };
    eprintln!("{}", msg);
    error_with!("Execution aborted")
}

fn delete_file(args: &[RhizValue]) -> ExecutionResult {
    use std::fs;
    use std::path::Path;
    check_args_len!("delete-file", args, 1);
    let fname = if let RhizValue::String(fname) = &args[0] {
        fname
    } else {
        error_with!("`delete-file` takes a string");
    };
    let fpath = Path::new(fname);
    fs::remove_file(fpath)?;
    Ok(())
}
