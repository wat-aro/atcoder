use std::{char::from_digit, iter::FromIterator};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h]
    }
    let dx: [i64; 3] = [-1, 0, 1];
    let dy: [i64; 3] = [-1, 0, 1];

    for i in 0..s.len() {
        for j in 0..s[i].len() {
            if s[i][j] == '.' {
                let mut count = 0;
                for xx in dx.iter() {
                    for yy in dy.iter() {
                        let x = j as i64 + xx;
                        let y = i as i64 + yy;
                        if 0 <= x
                            && x < w as i64
                            && 0 <= y
                            && y < h as i64
                            && s[y as usize][x as usize] == '#'
                        {
                            count += 1;
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
