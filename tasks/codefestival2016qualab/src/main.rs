use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut count = 0;
    for i in 0..n {
        if a[a[i] - 1] == i + 1 {
            count += 1;
        }
    }
    println!("{}", count / 2);
}
