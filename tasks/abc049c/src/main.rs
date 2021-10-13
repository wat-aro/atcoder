// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String
    }
    if solve(s) {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn solve(s: String) -> bool {
    let targets = ["dream", "dreamer", "erase", "eraser"];
    let mut dp: [bool; 100_000] = [false; 100_000];
    dp[0] = true;

    for i in 0..s.len() {
        if !dp[i] {
            continue;
        }

        for &target in targets.iter() {
            let length = target.len();
            if i + length > s.len() {
                continue;
            } else if &s[i..i + target.len()] == target {
                dp[i + target.len()] = true;
            }
        }
    }

    dp[s.len()]
}
