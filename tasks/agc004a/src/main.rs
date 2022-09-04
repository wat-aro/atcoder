use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64
    }
    if [a, b, c].iter().any(|x| x % 2 == 0) {
        println!("0");
    } else {
        let answer = (a * b).min(b * c).min(a * c);
        println!("{}", answer);
    }
}
