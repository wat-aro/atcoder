use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        n: usize,
         a: [usize; n]
    }
    let mut v = vec![0u64; n + 1];
    let mut total = 0u64;
    for ai in a.iter() {
        total += v[*ai];
        v[*ai] += 1;
    }
    let mut buf = String::new();
    for ai in a.iter() {
        let n = total - (v[*ai] - 1);
        writeln!(buf, "{}", n).unwrap();
    }
    print!("{}", buf);
}
