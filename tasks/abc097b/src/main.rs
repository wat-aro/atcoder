use proconio::input;

fn main() {
    input! {
        x: u64
    }
    let v: Vec<u64> = (1..=31)
        .map(|i: u64| (2..=10).map(move |j| i.pow(j)))
        .flatten()
        .filter(|&i| i <= x)
        .collect();
    let answer: &u64 = v.iter().filter(|&&y| y <= x).max().unwrap();
    println!("{}", answer);
}
