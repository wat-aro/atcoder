use cli_test_dir::*;

const BIN: &'static str = "./arc099a";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"4 3
2 3 1 4
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 3
1 2 3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"8 3
7 3 1 8 4 6 2 5
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4\n");
    assert!(output.stderr_str().is_empty());
}
