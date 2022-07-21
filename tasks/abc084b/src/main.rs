use proconio::input;

fn main() {
    input! {
        (a, b): (u8, u8),
        s: String
    }

    let v: Vec<&str> = s.split("-").collect();
    if v.len() == 2 && v[0].len() == a as usize && v[1].len() == b as usize {
        println!("Yes");
    } else {
        println!("No");
    }
}
