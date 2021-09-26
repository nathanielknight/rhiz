use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use rayon::prelude::*;

use crate::ast::RhizValue;
use crate::executor::{exec_sexpr, ExecutionError, ExecutionResult};

type RhizFunction = dyn Fn(&[RhizValue], &Path) -> ExecutionResult;

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
        RhizValue::String(s) => s,
        _ => return None,
    };
    match symbol_name.as_ref() {
        "log" => Some(Box::new(log)),
        "exec" => Some(Box::new(exec)),
        "empty-dir" => Some(Box::new(empty_dir)),
        "delete" => Some(Box::new(delete)),
        "copy" => Some(Box::new(copy)),
        "rec-copy" => Some(Box::new(rec_copy)),
        "par" => Some(Box::new(par)),
        _ => None,
    }
}

/// Execute tasks in parallel
/// (par (some other task), (some other task))
fn par(args: &[RhizValue], cwd: &Path) -> ExecutionResult {
    args.par_iter()
        .map(|arg| match arg {
            RhizValue::SExpr(x) => exec_sexpr(x, cwd),
            _ => error_with!("`par` needs sexprs!"),
        })
        .fold(
            || Ok(()),
            |acc, elem| match (&acc, &elem) {
                (Ok(_), Ok(_)) => Ok(()),
                (Err(_), _) => acc,
                (Ok(_), Err(_)) => elem,
            },
        )
        .collect()
}

fn join_cwd(cwd: &Path, fpath: &str) -> PathBuf {
    let mut cwd = cwd.to_path_buf();
    cwd.push(fpath);
    cwd
}

fn val_to_string(rval: &RhizValue) -> Option<String> {
    match rval {
        RhizValue::String(s) => Some(s.to_owned()),
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

/// Execute an external command
fn exec(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    assert!(working_dir.is_dir());
    if args.is_empty() {
        error_with!("`exec` needs at least one argument");
    }

    let cmd_name = val_to_string(&args[0]).ok_or_else(|| ExecutionError::from(
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
    let exit_code = child_process.wait()?;
    if !exit_code.success() {
        let mut cmd_string = String::new();
        cmd_string.push_str(&String::from(&args[0]));
        for arg in &args[1..] {
            cmd_string.push(' ');
            cmd_string.push_str(&String::from(arg));
        }
        let code_msg = match exit_code.code() {
            None => String::new(),
            Some(c) => format!("(exit code: {})", c),
        };

        error_with!(
            "External command returned an error code: {} {}",
            cmd_string,
            code_msg
        );
    }

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

/// Delete a file (by absolute path, or path relative to the Rhizfile).
fn delete(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    assert!(working_dir.is_dir());
    check_args_len!("delete", args, 1);
    let fpath = get_arg!("delete", args, 0, RhizValue::String);

    let target_path = join_cwd(working_dir, fpath);

    if target_path.exists() {
        fs::remove_file(target_path)?;
    }

    Ok(())
}

/// Copy a file (won't overwrite an existing file).
fn copy(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    assert!(working_dir.is_dir());
    check_args_len!("copy", args, 2);

    let src = get_arg!("copy", args, 0, RhizValue::String);
    let target = get_arg!("copy", args, 1, RhizValue::String);

    let src_path = Path::new(src);
    if !(src_path.exists()) {
        error_with!("`copy`'s source argument ({}) doesn't exist", src);
    }
    if !(src_path.is_file()) {
        error_with!("`copy` only acts on files ({} is not a file)", src);
    }

    let target_path_buf = {
        let arg_path = Path::new(target);
        if arg_path.exists() && arg_path.is_dir() {
            let mut t = arg_path.to_path_buf();
            let target_filename = src_path
                .file_name()
                .ok_or_else(|| ExecutionError::from("`copy` source doesn't have a file name?"))?;
            t.push(target_filename);
            t
        } else {
            arg_path.to_owned()
        }
    };
    let target_path = target_path_buf.as_path();
    assert!(!target_path.is_dir());
    if target_path.exists() {
        error_with!("`copy` won't clobber an existing file ({} exists)", target);
    }

    fs::copy(src_path, target_path)?;

    Ok(())
}

mod copy_tools {
    use std::fs;
    use std::path::Path;

    use crate::executor::{ExecutionError, ExecutionResult};

    fn copy_to(fpath: &Path, target_dir: &Path) -> ExecutionResult {
        assert!(fpath.exists() && fpath.is_file());
        assert!(target_dir.exists() && target_dir.is_dir());

        let mut target_dir_buf = target_dir.to_path_buf();
        let target_fname = fpath
            .file_name()
            .ok_or_else(|| ExecutionError::from("File doesn't have a file name?"))?;
        target_dir_buf.push(target_fname);
        let target_path = target_dir_buf.as_path();

        fs::copy(fpath, target_path)?;

        Ok(())
    }

    pub fn copy_dir(dirpath: &Path, target_path: &Path) -> ExecutionResult {
        assert!(dirpath.exists() && dirpath.is_dir());
        assert!(target_path.exists() && target_path.is_dir());

        for entry_r in fs::read_dir(dirpath)? {
            let entry = entry_r?;
            let meta = entry.metadata()?;
            if meta.is_file() {
                copy_to(&entry.path(), target_path)?;
            } else if meta.is_dir() {
                let target_dir_path = {
                    let mut target_pathbuf = target_path.to_path_buf();
                    target_pathbuf.push(entry.file_name());
                    target_pathbuf
                };
                fs::create_dir(&target_dir_path)?;
                copy_dir(&entry.path(), &target_dir_path)?;
            } else {
                error_with!("'{}' isn't a file or a directory?", entry.path().display());
            }
        }

        Ok(())
    }
}

/// Recursively copy one directory's contents into another.
fn rec_copy(args: &[RhizValue], working_dir: &Path) -> ExecutionResult {
    assert!(working_dir.is_dir());
    check_args_len!("rec-copy", args, 2);
    let src = get_arg!("rec-copy", args, 0, RhizValue::String);
    let target = get_arg!("rec-copy", args, 1, RhizValue::String);

    let src_path = working_dir.join(Path::new(src));
    if !src_path.exists() {
        error_with!("source directory doesn't exist ({})", src_path.display());
    }
    if !src_path.is_dir() {
        error_with!(
            "source directory isn't a directory ({})",
            src_path.display()
        );
    }
    let target_path = working_dir.join(Path::new(target));
    if !target_path.exists() {
        error_with!("target directory doesn't exist ({})", target_path.display());
    }
    if !target_path.is_dir() {
        error_with!(
            "target directory isn't a directory ({})",
            target_path.display()
        );
    }

    copy_tools::copy_dir(&src_path, &target_path)?;

    Ok(())
}
