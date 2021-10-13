use proconio::input;

fn main() {
    input! {
        (h, w): (f64, f64)
    }

    let epsilon = std::f64::EPSILON;
    let answer = if (h - 1.0).abs() < epsilon || (w - 1.0).abs() < epsilon {
        1.0
    } else if (h * w) as u64 % 2 == 0 {
        h * w / 2.0
    } else {
        h * w / 2.0 + 1.0
    };

    println!("{}", answer as u64);
}
