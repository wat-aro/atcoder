use cli_test_dir::*;

const BIN: &'static str = "./abc002a";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1 3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Positive\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"-3 -1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Negative\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"-1 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Zero\n");
    assert!(output.stderr_str().is_empty());
}
