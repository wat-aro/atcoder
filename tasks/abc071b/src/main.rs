use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut alpha = [0; 26];
    for c in s.chars() {
        alpha[c as usize - 97] += 1;
    }
    let mut zeros = alpha
        .iter()
        .enumerate()
        .filter(|(_index, value)| **value == 0);
    if let Some((index, _)) = zeros.next() {
        let answer = (index as u8 + 97) as char;
        println!("{}", answer);
    } else {
        println!("None");
    }
}
