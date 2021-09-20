// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        (_n, a, b): (u32, u32, u32),
        s: String
    }

    let mut grade = 0;
    let mut abroard_grade = 0;

    let answers = s.chars().map(|c| {
        if c == 'a' {
            if grade < a + b {
                grade = grade + 1;
                "Yes"
            } else {
                "No"
            }
        } else if c == 'b' {
            if grade < a + b && abroard_grade < b {
                grade = grade + 1;
                abroard_grade = abroard_grade + 1;
                "Yes"
            } else {
                "No"
            }
        } else {
            "No"
        }
    });

    println!("{}", answers.collect::<Vec<&str>>().join("\n"));
}
