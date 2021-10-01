use proconio::input;

fn main() {
    input! {
        (n, k): (i64, i64)
    }

    let x = n % k;

    let answer = std::cmp::min(x, (x - k).abs());

    println!("{}", answer);
}
