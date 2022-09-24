use proconio::input;

fn main() {
    input! {
        rgb: [usize; 3],
        n: usize
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for c in rgb {
        for i in 0..=n {
            if i >= c {
                dp[i] += dp[i - c]
            }
        }
    }
    println!("{}", dp[n]);
}
