use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let total: i64 = a.iter().sum();
    let mut sum = 0;
    let answer = a.iter().fold(total, |acc, x| {
        sum += x;
        acc.min((total - 2 * sum).abs())
    });
    println!("{}", answer);
}
