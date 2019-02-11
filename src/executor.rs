pub type ExecutionError = Box<std::error::Error>;
pub type ExecutionResult = Result<(), ExecutionError>;

pub trait Executable {
    fn run(&self) -> ExecutionResult;
}
