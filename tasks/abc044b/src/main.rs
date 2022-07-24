use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut v = vec![0; 26];
    for c in s.chars() {
        v[(c as u8 - 97) as usize] += 1;
    }
    let answer = if v.iter().all(|i| i % 2 == 0) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", answer);
}
