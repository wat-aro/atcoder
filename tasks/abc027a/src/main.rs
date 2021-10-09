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

fn solve(mut x: u32, v: &Vec<u32>) -> u32 {
    for (i, &t) in v.iter().enumerate() {
        if x == t {
            return i as u32 + 1;
        } else if x < t {
            return i as u32;
        } else {
            x = x - t;
        }
    }
    return v.len() as u32 - 1;
}
