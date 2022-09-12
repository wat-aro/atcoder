use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut max: u64 = 0;
    for i in 1..(n - 1) {
        let mut count = 0;
        for c in (b'a'..=b'z').map(char::from) {
            if s[0..i].iter().any(|c1| c == *c1) && s[i..n].iter().any(|c1| c == *c1) {
                count += 1;
            }
        }
        max = max.max(count);
    }

    println!("{}", max);
}
