use cli_test_dir::*;

const BIN: &'static str = "./abc104b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"AtCoder
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "AC\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"ACoder
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"AcycliC
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"AtCoCo
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"Atcoder
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}
