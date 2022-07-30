use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        n: usize,
        t: [u64; n],
        m: usize,
        p: [(u64, u64); m]
    }
    let sum: u64 = t.iter().sum();
    let mut buf = String::new();
    p.iter().for_each(|(pm, xm)| {
        let x = sum - t[*pm as usize - 1] + xm;
        writeln!(buf, "{}", x).unwrap();
    });

    print!("{}", buf);
}
