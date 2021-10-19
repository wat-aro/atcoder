use proconio::input;

fn main() {
    input! {
        h: u64
    }
    let beki = beki(h);
    let answer: u64 = (0..=beki).map(|i| 2_u64.pow(i)).sum();

    println!("{}", answer);
}

// n を超えない最も大きい2のべき乗
fn beki(n: u64) -> u32 {
    let mut i = 0;
    while n >= (2 as u64).pow(i + 1) {
        i += 1;
    }
    i
}
