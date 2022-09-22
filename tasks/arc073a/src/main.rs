use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        ts: [usize; n]
    }
    let mut count = 0;
    for i in 0..ts.len() {
        if i < ts.len() - 1 {
            count += t.min(ts[i + 1] - ts[i]);
        } else {
            count += t;
        }
    }
    println!("{}", count);
}
