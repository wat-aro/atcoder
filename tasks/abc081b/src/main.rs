// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [u32; n],
    }
    let mut x = 0;
    while v.iter().all(|i| i % 2 == 0) {
        x += 1;
        v = v.iter().map(|i| i / 2).collect();
    }

    println!("{}", x);
}
