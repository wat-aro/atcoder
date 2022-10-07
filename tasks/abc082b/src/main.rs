use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars
    }
    s.sort();
    t.sort();
    if s.first() < t.last() {
        println!("Yes");
    } else if s.iter().all(|c| t.iter().any(|c2| c == c2)) && s.len() < t.len() {
        println!("Yes");
    } else {
        println!("No");
    }
}
