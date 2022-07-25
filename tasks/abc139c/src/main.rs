use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [u64; n]
    }
    h.reverse();
    let mut dp = vec![0; n];
    for i in 1..n {
        if h[i - 1] <= h[i] {
            dp[i] = dp[i - 1] + 1
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
