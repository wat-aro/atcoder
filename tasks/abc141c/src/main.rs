use proconio::input;
use std::fmt::Write;

fn main() {
    input! {
        (n, k, q): (u64, i64, i64),
        questions: [usize; q],
    }
    let mut challengers: Vec<i64> = vec![k - q; n as usize];
    for q in questions {
        challengers[q - 1] += 1;
    }
    let mut buf = String::new();
    for c in challengers {
        if c > 0 {
            writeln!(buf, "Yes").unwrap();
        } else {
            writeln!(buf, "No").unwrap();
        }
    }

    print!("{}", buf);
}
