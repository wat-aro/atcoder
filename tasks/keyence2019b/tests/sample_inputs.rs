use cli_test_dir::*;

const BIN: &'static str = "./keyence2019b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"keyofscience
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
            r#"mpyszsbznf
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
            r#"ashlfyha
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "NO\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"keyence
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "YES\n");
    assert!(output.stderr_str().is_empty());
}
