use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut h: HashMap<char, u8> = HashMap::new();
    let mut answer: String = String::from("yes");
    for c in s.chars() {
        if let Some(_) = h.get(&c) {
            answer = String::from("no");
            break;
        } else {
            h.insert(c, 1);
        }
    }

    println!("{}", answer);
}
