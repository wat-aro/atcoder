use cli_test_dir::*;

const BIN: &'static str = "./abc064c";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"4
2100 2500 2700 2700
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2 2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5
1100 1900 2800 3200 3200
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3 5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"20
800 810 820 830 840 850 860 870 880 890 900 910 920 930 940 950 960 970 980 990
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1
3200
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2
800 3200
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample6() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2
3200 3200
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample7() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5
3200 3200 3200 3200 3200
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample8() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"9
3200 3200 3200 3200 3200 3200 3200 3200 3200
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 9\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample9() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"9
1 401 801 1201 1601 2001 2401 2801 3201
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "8 9\n");
    assert!(output.stderr_str().is_empty());
}
