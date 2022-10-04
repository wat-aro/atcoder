use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
        t: String
    }
    let mut i = 0;
    while i < n {
        if s[i..] == t[0..(n - i)] {
            break;
        } else {
            i += 1;
        }
    }
    let answer = n + i;
    println!("{}", answer);
}
