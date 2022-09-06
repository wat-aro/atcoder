use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut count = 0;
    let mut i = 0;
    let mut dp = vec![false; n];
    while !dp[i] {
        count += 1;
        if a[i] == 2 {
            println!("{}", count);
            return;
        } else {
            dp[i] = true;
            i = a[i] - 1;
        }
    }
    println!("-1");
}
