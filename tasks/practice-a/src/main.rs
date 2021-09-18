// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        str: String,
    }
    println!("{} {}", a + b + c, str);
}
