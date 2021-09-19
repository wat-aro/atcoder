// -*- coding:utf-8-unix -*-

use std::process::exit;

use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32
    }
    (0..=y / 10000).rev().for_each(|i| {
        (0..=((y - 10000 * i) / 5000)).rev().for_each(|j| {
            let k = (y - 10000 * i - 5000 * j) / 1000;
            if (i + j + k) == n {
                println!("{} {} {}", i, j, k);
                exit(0);
            }
        })
    });

    println!("{} {} {}", -1, -1, -1);
}
