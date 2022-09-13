use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n]
    }
    let mut joker = 0;
    let mut dp = vec![0u32; 8];
    for ai in a {
        if ai < 400 {
            dp[0] += 1;
        } else if ai < 800 {
            dp[1] += 1;
        } else if ai < 1200 {
            dp[2] += 1;
        } else if ai < 1600 {
            dp[3] += 1;
        } else if ai < 2000 {
            dp[4] += 1;
        } else if ai < 2400 {
            dp[5] += 1;
        } else if ai < 2800 {
            dp[6] += 1;
        } else if ai < 3200 {
            dp[7] += 1;
        } else {
            joker += 1;
        }
    }
    let max = dp.iter().filter(|x| **x > 0).count() + joker;
    let min = dp.iter().filter(|x| **x > 0).count().max(1);
    println!("{} {}", min, max);
}
