use proconio::input;

fn main() {
    input! {
        d: u32,
        n: u64
    }
    let num = 100u64.pow(d);
    let answer;
    if n == 100 {
        answer = num * 101;
    } else {
        answer = num * n;
    }

    println!("{}", answer);
}
