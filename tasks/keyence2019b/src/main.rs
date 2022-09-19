use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let keyence = ['k', 'e', 'y', 'e', 'n', 'c', 'e'];
    let mut dp = vec![false; 7];
    // 前から
    for i in 0..s.len() {
        if let Some(c) = keyence.get(i) {
            if s[i] == *c {
                dp[i] = true;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    // 後ろから
    for i in 0..s.len() {
        let keyence_index = keyence.len() - i - 1;
        if let Some(c) = keyence.get(keyence_index) {
            if s[s.len() - i - 1] == *c {
                dp[keyence_index] = true;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    println!("{}", if dp.iter().all(|b| *b) { "YES" } else { "NO" });
}
