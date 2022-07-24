use cli_test_dir::*;

const BIN: &'static str = "./abc090b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"11009 11332
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"31415 92653
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "612\n");
    assert!(output.stderr_str().is_empty());
}
