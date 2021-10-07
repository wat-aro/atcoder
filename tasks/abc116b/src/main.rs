use proconio::input;

fn main() {
    input! {
        mut n: u32
    }

    let mut v: Vec<u32> = vec![n];
    let mut i = 1;
    loop {
        i = i + 1;
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };

        if v.contains(&n) {
            break;
        } else {
            v.push(n);
            continue;
        }
    }

    println!("{}", i);
}
