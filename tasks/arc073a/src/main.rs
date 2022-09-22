use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        ts: [usize; n]
    }
    let mut count = 0;
    let mut expire = 0;
    for ti in ts {
        if ti < expire {
            count += t - (expire - ti);
            expire = ti + t;
        } else {
            count += t;
            expire = ti + t;
        }
    }
    println!("{}", count);
}
