use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        (h, _w): (usize, usize),
        c: [String; h]
    }
    let mut buf = String::new();
    c.iter().for_each(|s| {
        writeln!(buf, "{}", s).unwrap();
        writeln!(buf, "{}", s).unwrap();
    });
    print!("{}", buf);
}
