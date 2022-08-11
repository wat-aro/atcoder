use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        ps: [(usize, String); m]
    }
    let mut v = vec![(false, 0); n];
    ps.iter().for_each(|(p, s)| {
        if !v[*p - 1].0 {
            if *s == "AC" {
                v[*p - 1].0 = true;
            } else {
                v[*p - 1].1 += 1;
            }
        }
    });
    let answer = v.iter().fold(
        (0, 0),
        |acc, (ac, wa)| {
            if *ac {
                (acc.0 + 1, acc.1 + wa)
            } else {
                acc
            }
        },
    );
    println!("{} {}", answer.0, answer.1);
}
