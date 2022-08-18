use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }
    let mut answer = 0;
    for mut i in a {
        while i % 2 == 0 {
            answer += 1;
            i /= 2;
        }
    }

    println!("{}", answer);
}
