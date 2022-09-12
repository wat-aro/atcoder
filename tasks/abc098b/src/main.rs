use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut max: u64 = 0;
    for i in 1..(n - 1) {
        let mut count = 0;
        let mut hs = HashSet::new();
        s[0..i].iter().for_each(|c| {
            hs.insert(c);
        });
        hs.iter().for_each(|c| {
            if s[i..n].iter().any(|c1| *c == c1) {
                count += 1;
            }
        });
        max = max.max(count);
    }

    println!("{}", max);
}
