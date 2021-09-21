use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    let begin = ((n as f32) / 1.15) as u32;
    let end = ((n as f32) / 1.00) as u32;
    let answer: Option<u32> = (begin..=end).find(|&i| ((i as f32 * 1.08) as u32) == n);

    match answer {
        Some(m) => println!("{}", m),
        None => println!(":("),
    }
}
