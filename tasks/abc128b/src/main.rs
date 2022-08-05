yuse proconio::input;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::Write;

fn main() {
    input! {
        n: usize,
        mut sp: [(String, i64); n]
    }
    let mut sp: Vec<(usize, &(String, i64))> = sp.iter().enumerate().collect();
    sp.sort_by(|a, b| match (a.1).0.cmp(&(b.1).0) {
        Less => Less,
        Equal => match (a.1).1.cmp(&(b.1).1) {
            Less => Greater,
            Equal => Equal,
            Greater => Less,
        },
        Greater => Greater,
    });
    let mut buf = String::new();
    sp.iter().for_each(|x| {
        writeln!(buf, "{}", x.0 + 1).unwrap();
    });
    print!("{}", buf);
}
