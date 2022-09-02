use cli_test_dir::*;

const BIN: &'static str = "./codefestival2017qualcb";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2
2 3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "7\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3
3 3 3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "26\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1
100
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10
90 52 56 71 44 8 13 30 57 84
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "58921\n");
    assert!(output.stderr_str().is_empty());
}
