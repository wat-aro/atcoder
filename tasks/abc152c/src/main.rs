use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [u64; n]
    }
    let mut count = 0;
    v.iter().fold(std::u64::MAX, |min, i| {
        if *i <= min {
            count += 1;
            *i
        } else {
            min
        }
    });
    println!("{}", count);
}
