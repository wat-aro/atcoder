// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        n: usize,
        mut xs: [i32; n],
    }

    xs.sort();

    let &first = xs.first().unwrap();
    let &last = xs.last().unwrap();
    let answer = (first..=last)
        .map(|i| xs.iter().fold(0, |acc, &x| acc + (x - i).pow(2)))
        .min()
        .unwrap();
    println!("{}", answer);
}
