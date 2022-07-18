use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let length = n.len();

    let answer: i64 = (0..(length - 2))
        .map(|i| {
            let str = &n[i..(i + 3)];
            let number = str.parse::<i64>().unwrap();
            (753 - number).abs()
        })
        .min()
        .unwrap();
    println!("{}", answer);
}
