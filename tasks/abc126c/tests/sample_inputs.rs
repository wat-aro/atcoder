use cli_test_dir::*;

const BIN: &'static str = "./abc126c";

// #[test]
// fn sample1() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(
//             r#"3 10
// "#,
//         )
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "0.145833333333\n");
//     assert!(output.stderr_str().is_empty());
// }

// #[test]
// fn sample2() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(
//             r#"100000 5
// "#,
//         )
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "0.999973749998\n");
//     assert!(output.stderr_str().is_empty());
// }
