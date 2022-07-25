use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [u64; n]
    }
    let mut dp = vec![0; n];
    for i in (0..n - 1).rev() {
        if h[i] >= h[i + 1] {
            dp[i] = dp[i + 1] + 1
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
