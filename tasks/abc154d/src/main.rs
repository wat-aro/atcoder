use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }
    let mut v = vec![0f64; n];
    for i in 0..n {
        v[i] = (1..=p[i]).sum::<usize>() as f64 / p[i] as f64;
    }
    let mut total = (0..k).map(|i| v[i]).sum::<f64>();
    let mut answer = total;
    for i in k..n {
        total = total + v[i] - v[i - k];
        answer = answer.max(total);
    }
    println!("{}", answer);
}
