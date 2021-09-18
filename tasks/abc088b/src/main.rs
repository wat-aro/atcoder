use std::cmp::Reverse;

// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        n: u16,
        mut v: [u16; n],
    }
    let mut alice: u16 = 0;
    let mut bob: u16 = 0;
    v.sort_by_key(|&x| Reverse(x));
    let length = v.len();

    (0..length)
        .step_by(2)
        .for_each(|i| alice += *v.get(i).unwrap());
    (1..length)
        .step_by(2)
        .for_each(|j| bob += *v.get(j).unwrap());

    println!("{}", alice - bob);
}
