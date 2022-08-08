use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        (n, _l): (usize, usize),
        mut s: [String; n]
    }

    s.sort();
    let mut buf = String::new();
    s.iter().for_each(|s| {
        write!(buf, "{}", s).unwrap();
    });

    println!("{}", buf);
}
