use proconio::input;

fn main() {
    input! {
        n: u16,
    }
    let mut x: u16 = 1;
    while x * 2 <= n {
        x *= 2;
    }
    println!("{}", x);
}
