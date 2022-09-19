use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [u64; n]
    }
    let sum: u64 = s.iter().sum();

    if sum % 10 == 0 {
        if let Some(min) = s.iter().filter(|x| **x % 10 != 0).min() {
            println!("{}", sum - min);
        } else {
            println!("0");
        }
    } else {
        println!("{}", sum);
    }
}
