use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n]
    }
    let mut s: HashSet<String> = HashSet::new();
    let mut answer = "Yes";
    for i in 0..w.len() {
        if i == 0 {
            s.insert(w[i].to_string());
        } else if s.contains(&w[i]) {
            answer = "No";
            break;
        } else if w[i].chars().nth(0).unwrap() != w[i - 1].chars().last().unwrap() {
            answer = "No";
            break;
        } else {
            s.insert(w[i].to_string());
        }
    }
    println!("{}", answer);
}
