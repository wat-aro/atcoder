use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64
    }
    let d = (1..=n)
        .map(|j| {
            let mut p = 0;
            while j as f64 * 2f64.powf(p as f64) < k as f64 {
                p += 1;
            }
            (1f64 / n as f64) * (1f64 / 2f64).powf(p as f64)
        })
        .collect::<Vec<_>>();
    println!("{}", d.iter().sum::<f64>());
}
