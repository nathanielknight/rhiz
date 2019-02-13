use std::path::{Path, PathBuf};
use std::process::Command;

use crate::ast::RhizValue;
use crate::executor::{ExecutionError, ExecutionResult};

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
        "delete-dir" => Some(Box::new(delete_dir)),
        "exec" => Some(Box::new(exec)),
        _ => None,
    }
}

fn join_cwd(cwd: &Path, fpath: &str) -> PathBuf {
    let mut cwd = cwd.to_path_buf();
    cwd.push(fpath);
    cwd
}

fn val_to_string(rval: &RhizValue) -> Option<String> {
    match rval {
        RhizValue::String(s) => Some(s.to_owned()),
        RhizValue::Symbol(s) => Some(s.to_owned()),
        _ => None,
    }
}

/// Print a message to the console.
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

/// Stop executing the Rhizfile, printing an error message.
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

/// Delete a file (by absolute path, or path relative to the Rhizfile).
fn delete_file(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    use std::fs;
    assert!(working_dir.is_dir());
    check_args_len!("delete-file", args, 1);
    let fpath = if let RhizValue::String(fpath) = &args[0] {
        fpath
    } else {
        error_with!("`delete-file` takes a string");
    };
    let target_path = join_cwd(working_dir, fpath);

    fs::remove_file(target_path)?;
    Ok(())
}

fn delete_dir(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    assert!(working_dir.is_dir());
    check_args_len!("delete-dir", args, 1);
    let dpath = if let RhizValue::String(dpath) = &args[0] {
        dpath
    } else {
        error_with!("`delete-dir` takes a string");
    };
    let target_path = join_cwd(working_dir, dpath);
    if !target_path.is_dir() {
        error_with!("`delete-dir` is for deleting directories");
    }

    std::fs::remove_dir_all(&target_path)?;

    Ok(())
}

fn exec(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    assert!(working_dir.is_dir());
    if args.is_empty() {
        error_with!("`exec` needs at least one argument");
    }
    let cmd_name = val_to_string(&args[0]).ok_or(ExecutionError::from(
        "`exec` takes a string or symbol as a command name",
    ))?;

    let mut cmd = Command::new(&cmd_name);
    if args.len() > 1 {
        let mut cmd_args = Vec::new();
        for arg in args.iter().skip(1) {
            match val_to_string(arg) {
                Some(s) => cmd_args.push(s),
                None => error_with!("`exec` takes strings or symbols as command arguments"),
            }
        }
        cmd.args(&cmd_args);
    }
    cmd.current_dir(working_dir);
    let mut child_process = cmd.spawn()?;
    child_process.wait()?;

    Ok(())
}
