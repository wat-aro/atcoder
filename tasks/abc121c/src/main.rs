use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: u64,
        mut ab: [(u64, u64); n]
    }
    ab.sort_by(|a, b| a.0.cmp(&b.0));
    let mut total = 0;
    for i in 0..n {
        let (price, count) = ab[i];
        if count < m {
            total += price * count;
            m -= count;
        } else {
            total += price * m;
            break;
        }
    }
    println!("{}", total);
}
