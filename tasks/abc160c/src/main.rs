use proconio::input;

fn main() {
    input! {
        (k, n): (u64, u64),
        a: [u64; n]
    }

    let mut min = a[n as usize - 1] - a[0];
    (1..n as usize).for_each(|i| {
        min = std::cmp::min(min, a[i - 1] + k - a[i]);
    });

    println!("{}", min);
}
