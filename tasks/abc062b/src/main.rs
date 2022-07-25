use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        (h, w): (u8, u8),
        a: [String; h],
    }
    let mut buf = String::new();
    writeln!(buf, "{}", "#".repeat((w + 2) as usize)).unwrap();
    for ai in a {
        write!(buf, "#").unwrap();
        write!(buf, "{}", ai).unwrap();
        writeln!(buf, "#").unwrap();
    }
    writeln!(buf, "{}", "#".repeat((w + 2) as usize)).unwrap();
    print!("{}", buf);
}
