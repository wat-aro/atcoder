use cli_test_dir::*;

const BIN: &'static str = "./abc060b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"7 5 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "YES\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 2 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "NO\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1 100 97
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "YES\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"40 98 58
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "YES\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"77 42 36
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "NO\n");
    assert!(output.stderr_str().is_empty());
}
