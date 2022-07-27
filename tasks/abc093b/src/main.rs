use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        (a, b, k): (i64, i64, i64)
    }
    let mut buf = String::new();
    if b - k < a + k {
        for i in a..=b {
            writeln!(buf, "{}", i).unwrap();
        }
    } else {
        for i in a..(a + k) {
            writeln!(buf, "{}", i).unwrap();
        }
        for i in (b - k + 1)..=b {
            writeln!(buf, "{}", i).unwrap();
        }
    }

    print!("{}", buf);
}
