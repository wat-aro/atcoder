use proconio::input;

fn main() {
    input! {
        (a, b, _c, k): (i64, i64, i64, i64)
    }
    let answer;
    if k % 2 == 0 {
        answer = a - b
    } else {
        answer = b - a
    }

    println!("{}", answer);
}
