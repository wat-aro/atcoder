use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n]
    }
    let num = 100 * 100;
    let mut dp = vec![vec![0usize; num + 1]; n + 1];
    for i in 0..n {
        dp[i][0] = 1;
    }
    for i in 0..n {
        for j in 1..=10000 {
            if j >= s[i] && dp[i][j - s[i]] > 0 {
                for k in (i + 1)..=n {
                    dp[k][j] = dp[i][j - s[i]];
                }
            }
        }
    }

    let answer = dp[n]
        .iter()
        .enumerate()
        .filter(|(i, &x)| x > 0 && i % 10 != 0)
        .max_by_key(|(i, _)| *i)
        .unwrap_or((0, &0))
        .0;
    println!("{}", answer);
}
