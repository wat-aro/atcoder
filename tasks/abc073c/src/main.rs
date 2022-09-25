use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut h: HashMap<usize, usize> = HashMap::new();
    for ai in a {
        *h.entry(ai).or_insert(0) += 1;
    }
    let answer = h.values().filter(|&x| x % 2 != 0).count();
    println!("{}", answer);
}
