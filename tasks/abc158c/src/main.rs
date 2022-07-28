use proconio::input;

fn main() {
    input! {
        (a, b): (f64, f64)
    }

    for i in ((a / 0.08) as u16)..=(((b + 1.0) * 10.0) as u16) {
        let i = i as f64;
        if (i * 0.08).floor() == a && (i * 0.1).floor() == b {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
