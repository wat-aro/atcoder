use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let max = (n as f64).sqrt() as u64;
    let mut v = Vec::new();

    for i in 1..=max {
        if n % i == 0 {
            v.push(i + n / i);
        }
    }

    println!("{}", v.iter().min().unwrap() - 2);
}
