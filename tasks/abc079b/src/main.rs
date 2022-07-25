use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut v: Vec<u64> = Vec::new();
    v.push(2);
    v.push(1);
    for i in 2..=n {
        v.push(v[i - 1] + v[i - 2]);
    }
    println!("{}", v[n]);
}
