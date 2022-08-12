use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; 3 * n]
    }
    a.sort();
    let mut iter = a.iter().rev();
    iter.next();

    let answer: usize = iter.step_by(2).take(n).sum();
    println!("{}", answer);
}
