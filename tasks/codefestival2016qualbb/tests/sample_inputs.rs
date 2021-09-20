use cli_test_dir::*;

const BIN: &'static str = "./codefestival2016qualbb";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10 2 3
abccabaabb
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "Yes\nYes\nNo\nNo\nYes\nYes\nYes\nNo\nNo\nNo\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"12 5 2
cabbabaacaba
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "No\nYes\nYes\nYes\nYes\nNo\nYes\nYes\nNo\nYes\nNo\nNo\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5 2 2
ccccc
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "No\nNo\nNo\nNo\nNo\n");
    assert!(output.stderr_str().is_empty());
}
