use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [u64; n - 1]
    }
    let mut answer = 0;
    answer += b[0];
    for i in 0..(n - 2) {
        answer += b[i].min(b[i + 1]);
    }
    answer += b[n - 2];
    println!("{}", answer);
}
