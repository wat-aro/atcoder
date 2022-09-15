use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut h: HashMap<i64, u64> = HashMap::new();
    for ai in a {
        *h.entry(ai - 1).or_insert(0) += 1;
        *h.entry(ai).or_insert(0) += 1;
        *h.entry(ai + 1).or_insert(0) += 1;
    }
    let answer = h.values().max().unwrap();
    println!("{}", answer);
}
