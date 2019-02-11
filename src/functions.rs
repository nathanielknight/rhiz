use crate::ast::RhizValue;
use crate::executor::{ExecutionError, ExecutionResult};


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

fn look_up_function(fname: &str) -> Option<Box<RhizFunction>> {
    unimplemented!()
}
