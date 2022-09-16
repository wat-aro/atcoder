use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(i64, i64); n],
        cd: [(i64, i64); m]
    }
    for i in 0..n {
        let (ai, bi) = ab[i];
        let d = cd
            .iter()
            .map(|(ci, di)| (ai - ci).abs() + (bi - di).abs())
            .enumerate();
        println!("{}", d.min_by(|x, y| x.1.cmp(&y.1)).unwrap().0 + 1);
    }
}
