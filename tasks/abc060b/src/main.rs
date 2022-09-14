use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64
    }
    for i in 1..=b {
        if a * i % b == c {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
