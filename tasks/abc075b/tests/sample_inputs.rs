use cli_test_dir::*;

const BIN: &'static str = "./abc075b";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 5
.....
.#.#.
.....
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "11211
1#2#1
11211\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 5
#####
#####
#####
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "#####
#####
#####\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6 6
#####.
#.#.##
####.#
.#..#.
#.##..
#.#...
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "#####3
#8#7##
####5#
4#65#2
#5##21
#4#310\n"
    );
    assert!(output.stderr_str().is_empty());
}
