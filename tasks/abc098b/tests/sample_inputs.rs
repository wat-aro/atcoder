use cli_test_dir::*;

const BIN: &'static str = "./abc098b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6
aabbca
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10
aaaaaaaaaa
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"45
tgxgdqkyjzhyputjjtllptdfxocrylqfqjynmfbfucbir
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "9\n");
    assert!(output.stderr_str().is_empty());
}
