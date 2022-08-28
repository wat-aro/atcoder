use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }
    let patterns: Vec<_> = (0..n).permutations(n).collect();

    let answer: f64 = patterns
        .iter()
        .map(|pattern| {
            let mut length = 0.0;
            for i in 0..(n - 1) {
                length += ((xy[pattern[i]].0 - xy[pattern[i + 1]].0).pow(2) as f64
                    + (xy[pattern[i]].1 - xy[pattern[i + 1]].1).pow(2) as f64)
                    .sqrt()
            }
            length
        })
        .sum::<f64>()
        / patterns.len() as f64;

    println!("{}", answer);
}
