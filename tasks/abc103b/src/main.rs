use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    }
    let tt = t.clone() + &t[..];
    for i in 0..s.len() {
        if s == tt[i..s.len() + i] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
