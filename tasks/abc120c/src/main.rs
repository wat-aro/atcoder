use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let count = n.chars().filter(|c| *c == '1').count();
    let answer = count.min(n.len() - count) * 2;
    println!("{}", answer);
}
