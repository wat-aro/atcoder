use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u64; n]; 2]
    }
    let mut dp = vec![vec![0; n], vec![0; n]];

    for i in 0..n {
        if i == 0 {
            dp[0][0] = a[0][0];
        } else {
            dp[0][i] = a[0][i] + dp[0][i - 1];
        }
    }

    for i in 0..n {
        if i == 0 {
            dp[1][0] = a[1][0] + dp[0][0];
        } else {
            dp[1][i] = a[1][i] + dp[0][i].max(dp[1][i - 1]);
        }
    }

    println!("{}", dp[1][n - 1]);
}
