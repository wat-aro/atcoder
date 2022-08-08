use cli_test_dir::*;

const BIN: &'static str = "./abc042b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 3
dxx
axx
cxx
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "axxcxxdxx\n");
    assert!(output.stderr_str().is_empty());
}
