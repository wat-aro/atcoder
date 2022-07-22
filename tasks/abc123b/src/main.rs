use proconio::input;

fn maxtime(i: &u16) -> u16 {
    (i + 9) / 10 * 10
}

fn remtime(i: &u16) -> u16 {
    maxtime(i) - i
}

fn main() {
    input! {
        v: [u16; 5],
    }
    let sum: u16 = v.iter().map(|i| maxtime(i)).sum();
    let max_remtime = v.iter().map(|i| remtime(i)).max().unwrap();
    println!("{}", sum - max_remtime);
}
