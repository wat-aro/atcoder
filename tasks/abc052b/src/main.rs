use proconio::input;

fn main() {
    input! {
        _n: u8,
        s: String
    }
    let mut x = 0;
    let mut max = x;
    for c in s.chars() {
        if c == 'I' {
            x += 1;
            if x > max {
                max = x;
            }
        } else {
            x -= 1;
        }
    }
    println!("{}", max);
}
