use proconio::input;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64
    }
    if (b - a) % 2 == 0 {
        println!("{}", (b - a) / 2);
    } else {
        let answer = (a - 1).min(n - b) + (b - a + 1) / 2;
        println!("{}", answer);
    }
}
