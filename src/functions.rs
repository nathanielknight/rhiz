use crate::ast::RhizValue;
use crate::executor::{ExecutionError, ExecutionResult};

type RhizFunction = Fn(&[RhizValue]) -> ExecutionResult;

macro_rules! error_with {
    ($fmt:expr, $($val: expr),*) => {
        {
            let msg = format!($fmt, $($val,)*);
            return Err(ExecutionError::from(msg));
        }
    }
}

#[test]
fn test_error_with() {
    fn do_err() -> ExecutionResult {
        error_with!("oh {}", "noes")
    }
    assert!(do_err().is_err());
    fn do_ok() -> ExecutionResult {
        if false {
            error_with!("{} {} {}", "plz", "to", "not");
        }
        Ok(())
    }
    assert!(do_ok().is_ok());
}

macro_rules! get_arg {
    ($fname: expr, $args:expr, $idx:expr, $kind:path) => {{
        let val = match $args.get($idx) {
            Some(v) => v,
            None => error_with!("Not enough arguments to {}", $fname),
        };
        match val {
            $kind(v) => v,
            _ => error_with!("{} wrong type (arg {})", $fname, $idx),
        }
    }};
}

#[test]
fn test_test_arg() {
    fn get_string_at_0(args: &[RhizValue]) -> ExecutionResult {
        let x = get_arg!("get_string_at_0", args, 0, RhizValue::String);
        Ok(())
    }
    assert!(get_string_at_0(&[RhizValue::String(String::from("Hello"))]).is_ok());
    assert!(get_string_at_0(&[]).is_err());
    assert!(get_string_at_0(&[RhizValue::Symbol(String::from("Oh no!"))]).is_err());
}

fn look_up_function(fname: &str) -> Option<Box<RhizFunction>> {
    unimplemented!()
}
