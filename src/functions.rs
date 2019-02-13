use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::ast::RhizValue;
use crate::executor::{ExecutionError, ExecutionResult};

type RhizFunction = Fn(&[RhizValue], &Path) -> ExecutionResult;

macro_rules! error_with {
    ($msg:expr $(, $p:expr)* ) => {
        {
            let msg = format!($msg$(, $p)*);
            return Err(ExecutionError::from(msg));
        }
    };
}

macro_rules! check_args_len {
    ( $fname:expr, $args:expr, $cnt:expr ) => {
        if $args.len() != $cnt {
            error_with!("`{}` takes {} argument(s)", $fname, $cnt)
        }
    };
}

macro_rules! get_arg {
    ( $fname:expr, $args:expr, $idx:expr, $kind:path) => {{
        let arg = match $args.get($idx) {
            Some(a) => a,
            None => error_with!("Expected `{}` to have at least {} arguments", $fname, $idx),
        };
        if let $kind(v) = arg {
            v
        } else {
            error_with!(
                "Expected argument {} to `{}` to be a {}",
                $idx,
                $fname,
                stringify!($kind)
            );
        }
    }};
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
        "empty-dir" => Some(Box::new(empty_dir)),
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
    let msg = get_arg!("log", args, 0, RhizValue::String);

    println!("{}", msg);

    Ok(())
}

/// Stop executing the Rhizfile, printing an error message.
fn abort(args: &[RhizValue], _: &Path) -> ExecutionResult {
    check_args_len!("abort", args, 1);
    let msg = get_arg!("abort", args, 0, RhizValue::String);

    eprintln!("{}", msg);

    error_with!("Execution aborted")
}

/// Delete a file (by absolute path, or path relative to the Rhizfile).
fn delete_file(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    assert!(working_dir.is_dir());
    check_args_len!("delete-file", args, 1);
    let fpath = get_arg!("delete_file", args, 0, RhizValue::String);

    let target_path = join_cwd(working_dir, fpath);

    fs::remove_file(target_path)?;

    Ok(())
}

fn delete_dir(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    assert!(working_dir.is_dir());
    check_args_len!("delete-dir", args, 1);
    let dpath = get_arg!("delete-dir", args, 0, RhizValue::String);

    let target_path = join_cwd(working_dir, dpath);

    if !target_path.is_dir() {
        error_with!("`delete-dir` is for deleting directories");
    }

    fs::remove_dir_all(&target_path)?;

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
    cmd.current_dir(working_dir);

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

    let mut child_process = cmd.spawn()?;
    child_process.wait()?;

    Ok(())
}

/// If a directory exists, empty it. If it doesn't, create it (and its parents, if necessary).
fn empty_dir(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    assert!(working_dir.is_dir());
    if args.is_empty() {
        error_with!("`empty-dir` needs an argument");
    }
    let dpath = get_arg!("empty-dir", args, 0, RhizValue::String);

    let target_path = join_cwd(working_dir, dpath);

    match (target_path.exists(), target_path.is_dir()) {
        (false, _) => {
            fs::create_dir_all(target_path)?;
        }
        (true, true) => {
            let contents = fs::read_dir(target_path)?;
            for child_r in contents {
                let child = child_r?;
                let meta = child.metadata()?;
                if meta.is_dir() {
                    fs::remove_dir_all(child.path())?;
                } else if meta.is_file() {
                    fs::remove_file(child.path())?;
                } else {
                    error_with!("'{}' isn't a directory or a file?", child.path().display());
                }
            }
        }
        (true, false) => {
            error_with!("`delete-dir` can't operate on a file");
        }
    };

    Ok(())
}
