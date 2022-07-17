use proconio::input;

fn main() {
    input! {
        (n, a, b): (u64, u64, u64)
    }

    if a == 0 {
        println!("0");
        return;
    }

    let x = n / (a + b);
    let y = n % (a + b);
    let z = if y > a { a } else { y };
    let answer = x * a + z;

    println!("{}", answer);
}
