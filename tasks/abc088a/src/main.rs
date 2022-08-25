use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }
    let mut answer = 0;
    let mut ai = x;

    while ai <= y {
        answer += 1;
        ai *= 2;
    }

    println!("{}", answer);
}
