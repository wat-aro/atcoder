// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        (r, g, b): (u16, u16, u16)
    }

    let yes = (100 * r + 10 * g + b) % 4 == 0;
    println!("{}", if yes { "YES" } else { "NO" })
}
