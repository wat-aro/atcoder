use proconio::input;

fn main() {
    input! {
        (a, b, m): (u64, u64, u64),
        an: [u64; a],
        bn: [u64; b],
        v: [(u64, u64, u64); m]
    }

    let min_discount = v
        .iter()
        .map(|(ai, bi, mi)| an[*ai as usize - 1] + bn[*bi as usize - 1] - *mi)
        .fold(1_000_000, std::cmp::min);
    let without_discount =
        an.iter().fold(&1_000_000, std::cmp::min) + bn.iter().fold(&1_000_000, std::cmp::min);

    println!("{}", std::cmp::min(min_discount, without_discount));
}
