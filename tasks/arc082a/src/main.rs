use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut v: Vec<usize> = vec![0; 100001];
    for ai in a {
        v[ai] += 1;
    }
    let mut max = 0;
    for i in 0..v.len() - 1 {
        let sum = if i == 0 {
            v[0] + v[1]
        } else {
            v[i - 1] + v[i] + v[i + 1]
        };
        max = max.max(sum);
    }
    println!("{}", max);
}
