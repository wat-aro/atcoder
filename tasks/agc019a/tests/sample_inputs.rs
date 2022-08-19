use cli_test_dir::*;

const BIN: &'static str = "./agc019a";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"20 30 70 90
3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "150\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10000 1000 100 10
1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "100\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10 100 1000 10000
1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "40\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"12345678 87654321 12345678 87654321
123456789
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1524157763907942\n");
    assert!(output.stderr_str().is_empty());
}
