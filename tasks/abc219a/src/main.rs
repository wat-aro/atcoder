// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        x: u8,
    }

    let answer = if x < 40 {
        (40 - x).to_string()
    } else if x < 70 {
        (70 - x).to_string()
    } else if x < 90 {
        (90 - x).to_string()
    } else {
        String::from("expert")
    };

    println!("{}", answer);
}
