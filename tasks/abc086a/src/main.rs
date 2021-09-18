// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    println!("{}", if (n * m) % 2 == 0 { "Even" } else { "Odd" });
}
