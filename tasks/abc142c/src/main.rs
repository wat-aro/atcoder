use proconio::input;

fn main() {
    input! {
        n: u64,
        a: [u64; n]
    }

    let mut v: Vec<(usize, &u64)> = a.iter().enumerate().collect();
    v.sort_by(|(_i, ai), (_j, aj)| ai.partial_cmp(aj).unwrap());

    let answer = v
        .iter()
        .map(|(i, _ai)| (i + 1).to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", answer);
}
