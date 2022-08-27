use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut answer: u64 = 1;
    for i in 1..=n {
        answer = (answer * i) % (10_u64.pow(9) + 7)
    }
    println!("{}", answer);
}
