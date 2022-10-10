use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        _a: [usize; n]
    }
    let answer = ((n - k) as f64 / (k - 1) as f64).ceil() as usize + 1;
    println!("{}", answer);
}
