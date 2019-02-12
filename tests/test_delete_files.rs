use rhiz::ast::parse_rhiz_program;
use rhiz::compiler::compile;
use rhiz::executor::exec_task;

#[test]
fn test_delete_file() {
    use std::fs;
    use std::path::Path;

    let task_name = "test-deletion";
    let fname = "test-target";
    let fpath = Path::new(fname);
    let src = format!(r#"(task "{}" (delete-file "{}"))"#, task_name, fname);

    println!("delete-file test source: {}", src);

    let parsed = parse_rhiz_program(&src).unwrap();
    let compiled = compile(&parsed).unwrap();

    let cwd_b = std::env::current_dir().unwrap();
    let cwd = cwd_b.as_path();

    fs::write(&fname, "test contents").unwrap();

    exec_task(task_name, &compiled, cwd).unwrap();

    assert!(!fpath.exists());
}
