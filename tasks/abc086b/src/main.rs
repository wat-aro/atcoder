use proconio::input;

fn main() {
    input! {
        (a, b): (u8, u8)
    }
    let n = format!("{}{}", a, b).parse::<f64>().unwrap();
    let m = n.sqrt();
    println!(
        "{}",
        if (m - (m as u64) as f64) < std::f64::EPSILON {
            "Yes"
        } else {
            "No"
        }
    );
}
