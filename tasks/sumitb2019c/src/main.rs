use proconio::input;

fn main() {
    input! {
        x: usize
    }
    if x < 100 {
        println!("0");
        return;
    }
    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for i in 100..=x {
        if dp[i - 100] || dp[i - 101] || dp[i - 102] || dp[i - 103] || dp[i - 104] || dp[i - 105] {
            dp[i] = true
        }
    }
    println!("{}", if dp[x] { "1" } else { "0" });
}
