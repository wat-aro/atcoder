// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (n, a, b): (u32, u32, u32)
    }
    let answer: u32 = (1..=n)
        .filter(|&i| {
            let x = i.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
            a <= x && x <= b
        })
        .sum();
    println!("{}", answer)
}
