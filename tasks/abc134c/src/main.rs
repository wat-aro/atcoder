use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        n: usize,
        mut v: [u64; n],
    }
    let mut left = Vec::new();
    left.push(v[0]);
    let mut right = Vec::new();
    right.push(v[n - 1]);

    for i in 1..n {
        left.push(left[i - 1].max(v[i]));
        right.push(right[i - 1].max(v[n - 1 - i]));
    }

    right.reverse();
    let mut buf = String::new();
    for i in 0..n {
        if i == 0 {
            writeln!(buf, "{}", right[1]).unwrap();
        } else if i == n - 1 {
            writeln!(buf, "{}", left[i - 1]).unwrap();
        } else {
            writeln!(buf, "{}", left[i - 1].max(right[i + 1])).unwrap();
        }
    }
    print!("{}", buf);
}
