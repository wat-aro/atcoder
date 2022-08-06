use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if i == s.len() - 1 {
            count += 1;
            break;
        }

        if s[i] == s[i + 1] {
            if i + 1 == s.len() - 1 {
                count += 1;
                break;
            } else {
                count += 2;
                i += 3;
            }
        } else {
            count += 1;
            i += 1;
        }
    }
    println!("{}", count);
}
