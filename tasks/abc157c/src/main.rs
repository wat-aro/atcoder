use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(usize, u8); m]
    }
    let mut v: Vec<Option<u8>> = vec![None; n];
    for i in 0..sc.len() {
        if let Some(x) = v[sc[i].0 - 1] {
            if x != sc[i].1 {
                println!("-1");
                return;
            }
        } else if n > 1 && sc[i].0 == 1 && sc[i].1 == 0 {
            println!("-1");
            return;
        } else {
            v[sc[i].0 - 1] = Some(sc[i].1);
        }
    }
    let mut answer: u32 = 0;
    for i in 0..v.len() {
        if let Some(x) = v[i] {
            answer += x as u32 * 10u32.pow((n - i - 1) as u32);
        } else if n > 1 && i == 0 {
            answer += 10u32.pow((n - 1) as u32);
        }
    }
    println!("{}", answer);
}
