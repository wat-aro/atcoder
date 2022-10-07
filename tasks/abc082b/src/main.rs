use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars
    }
    s.sort();
    t.sort();

    if s.iter().collect::<String>() < t.iter().rev().collect::<String>() {
        println!("Yes");
    } else {
        println!("No");
    }
}
