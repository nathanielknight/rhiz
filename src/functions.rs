use crate::ast::RhizValue;
use crate::executor::{ExecutionError, ExecutionResult};
use std::path::Path;

type RhizFunction = Fn(&[RhizValue], &Path) -> ExecutionResult;

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

fn log(args: &[RhizValue], _: &Path) -> ExecutionResult {
    check_args_len!("log", args, 1);
    let msg = if let RhizValue::String(s) = &args[0] {
        s
    } else {
        error_with!("`log` takes a string");
    };
    println!("{}", msg);
    Ok(())
}

fn abort(args: &[RhizValue], _: &Path) -> ExecutionResult {
    check_args_len!("abort", args, 1);
    let msg = if let RhizValue::String(s) = &args[0] {
        s
    } else {
        error_with!("`abort` takes a string");
    };
    eprintln!("{}", msg);
    error_with!("Execution aborted")
}

fn delete_file(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    use std::fs;
    check_args_len!("delete-file", args, 1);
    let fpath = if let RhizValue::String(fpath) = &args[0] {
        fpath
    } else {
        error_with!("`delete-file` takes a string");
    };
    let target_path = {
        let mut pbuf = working_dir.to_path_buf();
        pbuf.push(fpath);
        pbuf
    };
    fs::remove_file(target_path)?;
    Ok(())
}
