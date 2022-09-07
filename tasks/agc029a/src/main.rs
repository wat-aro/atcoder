use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut answer = 0;
    let mut white_count = 0;
    for (i, c) in s.chars().enumerate() {
        if c == 'W' {
            answer += i - white_count;
            white_count += 1;
        }
    }
    println!("{}", answer);
}
