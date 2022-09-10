use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let answer: u64 = (1..n).sum();
    println!("{}", answer);
}
