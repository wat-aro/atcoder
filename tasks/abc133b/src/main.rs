use proconio::input;

fn main() {
    input! {
        (n, d): (usize, usize),
        points: [[f64; d]; n]
    }
    let mut count = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let y = points[i].clone();
            let z = points[j].clone();
            let d: f64 = y
                .into_iter()
                .zip(z)
                .map(|(yd, zd)| (yd - zd).powf(2.0))
                .sum::<f64>()
                .abs()
                .sqrt();

            if d == d.floor() {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
