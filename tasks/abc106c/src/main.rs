use proconio::input;

fn main() {
    input! {
        s: String,
        k: usize
    }
    for i in 0..k {
        let c = s.chars().nth(i).unwrap();
        if c != '1' {
            println!("{}", c);
            return;
        }
    }

    println!("1");
}
