use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut v: Vec<u64> = vec![0; s.len() + 1];
    for i in 0..s.len() {
        if s[i] == '<' {
            v[i + 1] = v[i] + 1;
        }
    }
    for i in (0..s.len()).rev() {
        if s[i] == '>' && v[i] <= v[i + 1] {
            v[i] = v[i + 1] + 1;
        }
    }
    let answer: u64 = v.iter().sum();
    println!("{}", answer);
}
