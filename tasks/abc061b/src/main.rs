use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        (n, m): (usize, usize),
        v: [(usize, usize); m]
    }
    let mut roads = vec![0; n];
    v.iter().for_each(|(a, b)| {
        roads[a - 1] += 1;
        roads[b - 1] += 1;
    });

    let mut buf = String::new();
    roads.iter().for_each(|i| {
        writeln!(buf, "{}", i).unwrap();
    });
    print!("{}", buf);
}
