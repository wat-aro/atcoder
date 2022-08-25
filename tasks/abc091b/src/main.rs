use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
        m: usize,
        t: [String; m]
    }
    let mut h: HashMap<String, i64> = HashMap::new();
    s.iter().for_each(|string| {
        let count = h.entry(string.clone()).or_insert(0);
        *count += 1;
    });
    t.iter().for_each(|string| {
        let count = h.entry(string.clone()).or_insert(0);
        *count -= 1;
    });
    let answer = h.values().max().unwrap().max(&0);
    println!("{}", answer);
}
