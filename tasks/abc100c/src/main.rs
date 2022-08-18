use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    let answer: u64 = a.iter().map(|i| count2(*i)).sum();

    println!("{}", answer);
}

fn count2(n: u64) -> u64 {
    let mut count = 0;
    let mut m = n;
    while m % 2 == 0 {
        count += 1;
        m = m / 2;
    }
    count
}
