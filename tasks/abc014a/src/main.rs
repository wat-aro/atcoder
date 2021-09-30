use proconio::input;

fn main() {
    input! {
        (mut a, mut b, mut c): (u64, u64, u64)
    }

    if a % 2 == 0 && a == b && b == c {
        println!("-1");
        return;
    }

    let mut i = 0;
    while a % 2 == 0 && b % 2 == 0 && c % 2 == 0 {
        let t1 = a / 2;
        let t2 = b / 2;
        let t3 = c / 2;
        a = t1 + t2;
        b = t2 + t3;
        c = t3 + t1;
        i = i + 1;
    }

    println!("{}", i);
}
