use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    let answer = a.iter().fold(0, |acc, x| acc + x - 1);
    println!("{}", answer);
}
