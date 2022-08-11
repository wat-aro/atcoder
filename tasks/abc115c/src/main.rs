use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut h: [u64; n]
    }
    h.sort();

    let mut min = std::u64::MAX;
    for i in 0..=(n - k) {
        min = min.min(h[i + k - 1] - h[i]);
    }
    println!("{}", min);
}
