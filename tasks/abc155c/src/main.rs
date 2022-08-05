use std::fmt::Write;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n]
    }
    s.sort();
    let mut v = vec![0; n];
    let mut max = 0;
    for i in 0..(n - 1) {
        if s[i] == s[i + 1] {
            v[i + 1] = v[i] + 1;
            max = max.max(v[i + 1]);
        }
    }
    let mut buf = String::new();
    for i in 0..n {
        if v[i] == max {
            writeln!(buf, "{}", s[i]).unwrap();
        }
    }
    print!("{}", buf);
}
