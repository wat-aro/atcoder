use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    let mut number = 1;
    for i in 0..n {
        if number == a[i] {
            number += 1;
        }
    }
    if number == 1 {
        println!("-1");
    } else {
        println!("{}", n as u64 + 1 - number);
    }
}
