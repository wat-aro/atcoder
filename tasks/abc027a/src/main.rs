use proconio::input;

fn main() {
    input! {
        (n, x): (u32, u32),
        mut a: [u32; n],
    }

    a.sort();
    let answer = solve(x, &a);

    println!("{}", answer);
}

fn solve(mut x: u32, v: &[u32]) -> u32 {
    for (i, &t) in v.iter().enumerate() {
        match x.cmp(&t) {
            std::cmp::Ordering::Equal => {
                return i as u32 + 1;
            }
            std::cmp::Ordering::Greater => {
                x -= t;
            }
            std::cmp::Ordering::Less => {
                return i as u32;
            }
        }
    }
    v.len() as u32 - 1
}
