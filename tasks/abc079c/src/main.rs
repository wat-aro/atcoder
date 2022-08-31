use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        s: String
    }
    for op in 0..=7 {
        let mut buf = String::new();
        write!(buf, "{}", s.chars().nth(0).unwrap()).unwrap();
        let mut a = s.chars().nth(0).unwrap().to_digit(10).unwrap() as i64;
        for i in 0..3 {
            if op & (1 << i) != 0 {
                a += s.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i64;
                write!(buf, "{}{}", "+", s.chars().nth(i + 1).unwrap()).unwrap();
            } else {
                a -= s.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i64;
                write!(buf, "{}{}", "-", s.chars().nth(i + 1).unwrap()).unwrap();
            }
        }

        if a == 7 {
            println!("{}=7", buf);
            return;
        }
    }
}
