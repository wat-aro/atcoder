use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        answers: [[u16]; n],
    }
    let mut result = vec![0; m];
    for v in answers.iter() {
        for i in v {
            result[(i - 1) as usize] += 1;
        }
    }
    let answer = result.iter().filter(|i| **i == n).count();

    println!("{}", answer);
}
