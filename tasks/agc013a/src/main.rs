use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut sign: Option<char> = None;
    let mut count = 1;
    let mut prev = a[0];

    for i in 1..a.len() {
        match sign {
            Some('+') => {
                if a[i] - prev >= 1 || a[i] - prev == 0 {
                    prev = a[i];
                } else {
                    count += 1;
                    sign = None;
                    prev = a[i];
                }
            }
            Some('-') => {
                if prev - a[i] >= 1 || prev - a[i] == 0 {
                    prev = a[i];
                } else {
                    count += 1;
                    sign = None;
                    prev = a[i];
                }
            }
            Some(_) => unreachable!(),
            None => {
                if a[i] - prev >= 1 {
                    sign = Some('+');
                    prev = a[i];
                } else if prev - a[i] >= 1 {
                    sign = Some('-');
                    prev = a[i];
                } else if a[i] == prev {
                    prev = a[i];
                } else {
                    prev = a[i];
                }
            }
        }
    }
    println!("{}", count);
}
