use proconio::input;

fn main() {
    input! {
        (h, w): (f64, f64)
    }

    let answer = if h == 1.0 || w == 1.0 {
        1.0
    } else if (h * w) as u64 % 2 == 0 {
        h * w / 2.0
    } else {
        h * w / 2.0 + 1.0
    };

    println!("{}", answer as u64);
}
