use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64
    }
    let answer = if n == 1 && m == 1 {
        1
    } else if n == 1 {
        m - 2
    } else {
        (n - 2) * (m - 2)
    };

    println!("{}", answer);
}
