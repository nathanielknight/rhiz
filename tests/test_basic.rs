use rhiz::ast::parse_rhiz_program;
use rhiz::compiler::compile;
use rhiz::executor::exec_task;

#[test]
fn test_basic_compilation_and_execution() {
    let src = r#"(task "dummy-task" (log "This is a dummy task"))"#;
    let parsed = parse_rhiz_program(src).unwrap();
    let compiled = compile(&parsed).unwrap();
    let pbuf = std::env::current_dir().unwrap();
    let cwd = pbuf.as_path();
    exec_task("dummy-task", &compiled, &cwd).unwrap();
}
