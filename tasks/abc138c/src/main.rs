use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [i64; n]
    }
    v.sort();
    let answer: f64 = v
        .iter()
        .fold(v[0] as f64, |acc, &x| (acc + x as f64) / 2_f64);

    println!("{}", answer);
}
