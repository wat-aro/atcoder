use cli_test_dir::*;

const BIN: &'static str = "./diverta2019b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1 2 3 4
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"13 1 4 3000
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "87058\n");
    assert!(output.stderr_str().is_empty());
}
