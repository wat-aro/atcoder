use proconio::input;

fn main() {
    input! {
        n: u8,
        k: i32,
        x: [i32; n]
    }
    let answer: i32 = x
        .iter()
        .map(|&xi| std::cmp::min(xi * 2, (k - xi).abs() * 2))
        .sum();
    println!("{}", answer);
}
