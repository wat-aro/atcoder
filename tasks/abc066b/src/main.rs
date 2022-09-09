use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut length = 2;
    let mut g = 0;
    while length < s.len() {
        if s[0..(length / 2)] == s[(length / 2)..length] {
            g = length;
        }
        length += 2;
    }
    println!("{:?}", g);
}
