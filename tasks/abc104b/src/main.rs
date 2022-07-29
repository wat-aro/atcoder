use proconio::input;

fn main() {
    input! {
        s: String
    }
    if s.chars().nth(0).unwrap() != 'A' {
        println!("WA");
        return;
    }
    if s[2..(s.len() - 1)].chars().filter(|&c| c == 'C').count() != 1 {
        println!("WA");
        return;
    }
    if s.chars().filter(|c| c.is_uppercase()).count() != 2 {
        println!("WA");
        return;
    }

    println!("AC");
}
