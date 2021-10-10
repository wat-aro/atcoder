use proconio::input;

fn main() {
    input! {
        n: u8,
        (d, x): (u8, u8),
        a: [u8; n]
    }

    let all_eat: u32 = a.iter().map(|&ai| ((d - 1) / ai + 1) as u32).sum();

    println!("{}", all_eat + x as u32);
}
