use proconio::input;

fn main() {
    input! {
        o: String,
        e: String
    }
    let mut s = String::new();
    for i in 0..o.len() {
        s.push(o.chars().nth(i).unwrap());
        if let Some(c) = e.chars().nth(i) {
            s.push(c);
        }
    }
    println!("{}", s);
}
