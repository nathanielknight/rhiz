use crate::ast::RhizValue;
use crate::executor::{ExecutionError, ExecutionResult};

type RhizFunction = Fn(Vec<RhizValue>) -> ExecutionResult;

fn look_up_function(fname: &str) -> Option<Box<RhizFunction>> {
    unimplemented!()
}
