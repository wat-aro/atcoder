use cli_test_dir::*;

const BIN: &'static str = "./abc084b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 4
269-6650
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Yes\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1 1
---
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "No\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1 2
7444
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "No\n");
    assert!(output.stderr_str().is_empty());
}
