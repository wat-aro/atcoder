// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        n: usize
    }

    let x: usize = (0..=a)
        .collect::<Vec<usize>>()
        .iter()
        .flat_map(|&i| (0..=b).flat_map(move |j| (0..=c).map(move |k| 500 * i + 100 * j + 50 * k)))
        .filter(|&sum| sum == n)
        .count();

    println!("{}", x);
}
