use rhiz::ast::parse_rhiz_program;
use rhiz::compiler::compile;
use rhiz::executor::exec_task;

#[test]
fn test_basic_compilation_and_execution() {
    let src = r#"(task "dummy-task" (log "This is a dummy task"))"#;
    let parsed = parse_rhiz_program(src).unwrap();
    let compiled = compile(&parsed).unwrap();
    exec_task("dummy-task", compiled).unwrap();
}
