use cli_test_dir::*;

const BIN: &'static str = "./abc079b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "11\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"86
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "939587134549734843\n");
    assert!(output.stderr_str().is_empty());
}
