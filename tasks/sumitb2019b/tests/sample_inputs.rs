use cli_test_dir::*;

const BIN: &'static str = "./sumitb2019b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"432
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "400\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1079
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), ":(\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1001
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "927\n");
    assert!(output.stderr_str().is_empty());
}
