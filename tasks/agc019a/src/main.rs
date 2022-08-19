use proconio::input;

fn main() {
    input! {
        q: u64,
        h: u64,
        s: u64,
        d: u64,
        n: u64
    }

    let v = vec![4 * q, 2 * h, s];
    let min = v.iter().min().unwrap();

    if min * 2 < d {
        println!("{}", min * n);
    } else {
        let double = n / 2;
        let answer = d * double + min * (n % 2);
        println!("{}", answer);
    }
}
