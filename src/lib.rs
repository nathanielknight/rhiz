#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod compiler;
pub mod executor;
mod functions;
mod parser;
