use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        n: usize,
        v: [u64; n],
    }
    let (first, second) =
        v.iter()
            .enumerate()
            .fold(((0, 0), (0, 0)), |(first, second), (index, value)| {
                if first.1 < *value {
                    ((index, *value), first)
                } else if second.1 < *value {
                    (first, (index, *value))
                } else {
                    (first, second)
                }
            });
    let mut buf = String::new();
    for (i, _v) in v.iter().enumerate() {
        if i == first.0 {
            writeln!(buf, "{}", second.1).unwrap();
        } else {
            writeln!(buf, "{}", first.1).unwrap();
        }
    }
    print!("{}", buf);
}
