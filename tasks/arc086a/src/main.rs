use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut v = vec![0; n + 1];
    for ai in a {
        v[ai] += 1;
    }
    let mut v = v.iter().filter(|&&i| i > 0).collect::<Vec<_>>();

    if k >= v.len() {
        println!("0");
    } else {
        v.sort();
        let mut i = 0usize;
        let mut count = 0i64;
        while k + i < v.len() {
            count += *v[i] as i64;
            i += 1;
        }
        println!("{}", count);
    }
}
