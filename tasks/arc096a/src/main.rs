use proconio::input;

fn main() {
    input! {
        (a, b, c, x, y): (u64, u64, u64, u64, u64)
    }
    if a + b < 2 * c {
        let answer = a * x + b * y;
        println!("{}", answer);
    } else {
        let z = x.min(y);
        let answer = (a * (x - z) + b * (y - z) + c * z * 2).min(x.max(y) * c * 2);
        println!("{}", answer);
    }
}
