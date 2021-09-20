// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        (a, b): (u8, u8)
    }

    let answer = (b - 1 + a - 2) / (a - 1);

    println!("{}", answer);
}
