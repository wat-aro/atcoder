use cli_test_dir::*;

const BIN: &'static str = "./abc024a";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1 2 3 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 3 2 0
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1000000000 1000000000 1000000000 1000000000000000000
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}
