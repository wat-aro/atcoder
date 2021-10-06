use proconio::input;

fn main() {
    input! {
        (_n, m, x): (u8, u8, u8),
        a: [u8; m],
    }

    let mut small: Vec<u8> = Vec::new();
    let mut big: Vec<u8> = Vec::new();

    for i in a {
        if i < x {
            small.push(i);
        } else {
            big.push(i)
        }
    }

    let answer = std::cmp::min(small.iter().count(), big.iter().count());

    println!("{}", answer);
}
