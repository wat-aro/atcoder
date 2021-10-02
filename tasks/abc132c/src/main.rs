use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [u64; n]
    }

    v.sort();
    if v.len() % 2 == 1 {
        println!("0");
        return;
    }
    let half = v.len() / 2;
    println!("{}", v[half] - v[half - 1]);
}
