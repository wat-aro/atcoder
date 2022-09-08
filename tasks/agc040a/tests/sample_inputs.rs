use cli_test_dir::*;

const BIN: &'static str = "./agc040a";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"<>>
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"<>>><<><<<<<>>><
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "28\n");
    assert!(output.stderr_str().is_empty());
}
