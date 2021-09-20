// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut plan: [(i32, i32, i32); n],
    }

    plan.insert(0, (0, 0, 0));
    let yes = plan.windows(2).all(|w| {
        let (t1, x1, y1) = w[0];
        let (t2, x2, y2) = w[1];
        let time = t2 - t1;
        let dist = (x2 - x1).abs() + (y2 - y1).abs();

        dist <= time && dist % 2 == time % 2
    });

    println!("{}", if yes { "Yes" } else { "No" });
}
