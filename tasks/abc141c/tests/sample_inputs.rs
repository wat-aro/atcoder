use cli_test_dir::*;

const BIN: &'static str = "./abc141c";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6 3 4
3
1
3
2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "No
No
Yes
No
No
No\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6 5 4
3
1
3
2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "Yes
Yes
Yes
Yes
Yes
Yes\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10 13 15
3
1
4
1
5
9
2
6
5
3
5
8
9
7
9
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "No
No
No
No
Yes
No
No
No
Yes
No\n"
    );
    assert!(output.stderr_str().is_empty());
}
