use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n]
    }
    let count: usize = a.iter().count();
    let even: usize = a.iter().filter(|x| *x % 2 == 0).count();

    let answer: u64 = 3u64.pow(count as u32) - 2u64.pow(even as u32);

    println!("{}", answer);
}
