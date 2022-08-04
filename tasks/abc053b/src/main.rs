use proconio::input;

fn main() {
    input! {
        s: String
    }
    let a_index = s.chars().position(|c| c == 'A').unwrap();
    let z_index = s.bytes().rposition(|c| c as char == 'Z').unwrap();
    println!("{}", z_index - a_index + 1);
}
