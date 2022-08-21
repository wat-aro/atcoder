use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n+1],
        b: [usize; n]
    }

    let mut d = vec![0; n + 1];
    for i in 0..n {
        if a[i] > d[i] + b[i] {
            d[i] += b[i];
        } else {
            let diff = d[i] + b[i] - a[i];
            if diff > a[i + 1] {
                d[i] = a[i];
                d[i + 1] = a[i + 1];
            } else {
                d[i] = a[i];
                d[i + 1] = diff;
            }
        }
    }

    println!("{}", d.iter().sum::<usize>());
}
