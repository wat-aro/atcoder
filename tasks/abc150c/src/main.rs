use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }
    let mut p_i = 0i32;
    let mut q_i = 0i32;
    for (i, x) in (1..=n).permutations(n).enumerate() {
        if p_i != 0 && q_i != 0 {
            break;
        }
        if x == p {
            p_i = i as i32;
        }
        if x == q {
            q_i = i as i32;
        }
    }

    println!("{}", (p_i - q_i).abs());
}
