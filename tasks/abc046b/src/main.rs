use proconio::input;

fn main() {
    input! {
        n: u32,
        k: u32
    }
    let answer = k * (k - 1).pow(n - 1 as u32);
    println!("{}", answer);
}
