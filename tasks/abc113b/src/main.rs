use proconio::input;

fn main() {
    input! {
        n: usize,
        (t, a): (i64, i64),
        h: [i64; n],
    }
    let answer: usize = h
        .iter()
        .enumerate()
        .map(|(i, hi)| {
            let temp = (t as f64) - (*hi as f64) * 0.006;
            (i, (a as f64 - temp).abs())
        })
        .fold(
            (0, std::f64::MAX),
            |acc, (i, hi)| if acc.1 <= hi { acc } else { (i, hi) },
        )
        .0;
    println!("{}", answer + 1);
}
