use proconio::input;

fn main() {
    input! {
        s: String
    }
    let length = s.len();
    let first = s
        .chars()
        .nth(0)
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();

    let answer = if s == first.to_string() + &("9".repeat(length - 1)) {
        first + 9 * (length - 1)
    } else {
        first + 9 * (length - 1) - 1
    };

    println!("{}", answer);
}
