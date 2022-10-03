use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut str: Vec<char> = Vec::new();

    for c in s {
        match c {
            '0' | '1' => {
                str.push(c);
            }
            'B' => {
                str.pop();
            }
            _ => unreachable!(),
        }
    }

    println!("{}", str.iter().collect::<String>());
}
