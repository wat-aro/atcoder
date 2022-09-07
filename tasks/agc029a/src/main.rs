use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut answer: u64 = 0;
    let mut black_count: u64 = 0;
    for c in s.chars() {
        if c == 'W' {
            answer += black_count;
        } else {
            black_count += 1;
        }
    }
    println!("{}", answer);
}
