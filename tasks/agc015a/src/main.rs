use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }
    if a > b {
        println!("0");
        return;
    }
    if n == 1 {
        if a == b {
            println!("1");
            return;
        } else {
            println!("0");
            return;
        }
    }

    let min = b + a * (n - 1);
    let max = a + b * (n - 1);

    println!("{}", max - min + 1);
}
