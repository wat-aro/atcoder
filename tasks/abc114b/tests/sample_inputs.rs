use cli_test_dir::*;

const BIN: &'static str = "./abc114b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1234567876
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "34\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"35753
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1111111111
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "642\n");
    assert!(output.stderr_str().is_empty());
}
