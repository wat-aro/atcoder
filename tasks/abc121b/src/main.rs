// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        (n, m, c): (i32, i32, i32),
        b: [i32; m],
        a: [[i32; m]; n],
    }

    let answer: usize = a
        .iter()
        .map(|v: &Vec<i32>| v.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum::<i32>())
        .filter(|x| x + c > 0)
        .count();

    println!("{}", answer);
}
