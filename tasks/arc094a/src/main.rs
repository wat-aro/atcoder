use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }
    let max = a.max(b).max(c);
    let diff = 3 * max - a - b - c;

    let answer;
    if diff % 2 == 0 {
        answer = diff / 2;
    } else {
        answer = diff / 2 + 2;
    }
    println!("{}", answer);
}
