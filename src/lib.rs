/// # Rhiz
///
/// A deliberately minimal task manager.
///
/// Rhiz is a generic task runner. Tasks are defined in a `Rhizfile` using
/// s-expressions.
///
/// ```ignore
/// (task "build-dev"
///   "Build development artifacts."
///   (exec cargo build))
///
/// (task "build-release"
///   "Build release artifacts."
///   (exec cargo --release build))
///
/// (task "clean"
///   "Empty the 'target' directory."
///   (empty-dir ./target))
/// ```
#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod compiler;
pub mod executor;
mod functions;
mod parser;
