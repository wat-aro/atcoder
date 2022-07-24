use proconio::input;

fn main() {
    input! {
        (a, b): (i64, i64)
    }
    let answer = if 0 < a {
        "Positive"
    } else if a <= 0 && 0 <= b {
        "Zero"
    } else {
        if (b - a) % 2 == 0 {
            "Negative"
        } else {
            "Positive"
        }
    };

    println!("{}", answer);
}
