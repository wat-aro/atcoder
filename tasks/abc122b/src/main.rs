use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let targets: Vec<char> = "ACGT".chars().collect();
    let mut count: u8 = 0;
    let mut result: Vec<u8> = Vec::new();
    for c in s.chars() {
        if targets.iter().any(|&t| t == c) {
            count += 1;
        } else {
            result.push(count);
            count = 0;
        }
    }
    result.push(count);

    let answer = result.iter().max().unwrap_or(&0);
    println!("{}", answer);
}
