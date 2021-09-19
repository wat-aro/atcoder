// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [u8; n]
    }
    let mut vec: Vec<u8> = vec![0; 100];
    v.iter().for_each(|&i| {
        vec[(i - 1) as usize] += 1;
    });
    println!("{}", vec.iter().filter(|&&n| n > 0).count());
}
