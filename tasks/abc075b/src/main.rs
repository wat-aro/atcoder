use std::{char::from_digit, iter::FromIterator};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h]
    }
    for i in 0..s.len() {
        for j in 0..s[i].len() {
            if s[i][j] == '.' {
                let mut count: u32 = 0;
                if i > 0 {
                    if s[i - 1][j] == '#' {
                        count += 1;
                    }
                    if j > 0 {
                        if s[i - 1][j - 1] == '#' {
                            count += 1;
                        }
                    }
                    if j < w - 1 {
                        if s[i - 1][j + 1] == '#' {
                            count += 1;
                        }
                    }
                }
                if j > 0 {
                    if s[i][j - 1] == '#' {
                        count += 1;
                    }
                }
                if j < w - 1 {
                    if s[i][j + 1] == '#' {
                        count += 1;
                    }
                }
                if i < h - 1 {
                    if s[i + 1][j] == '#' {
                        count += 1;
                    }

                    if j > 0 {
                        if s[i + 1][j - 1] == '#' {
                            count += 1;
                        }
                    }
                    if j < w - 1 {
                        if s[i + 1][j + 1] == '#' {
                            count += 1
                        }
                    }
                }
                s[i][j] = from_digit(count, 10).unwrap();
            }
        }
    }

    for i in 0..s.len() {
        println!("{}", String::from_iter(s[i].clone()));
    }
}
