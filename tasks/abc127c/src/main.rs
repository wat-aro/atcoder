use proconio::input;

fn main() {
    input! {
        (n, m): (u64, u64),
        v: [(u64, u64); m]
    }
    let (l, r) = v.into_iter().fold((1, n), |(accl, accr), (l, r)| {
        let accl = accl.max(l);
        let accr = accr.min(r);
        (accl, accr)
    });
    if l <= r {
        println!("{}", r - l + 1);
    } else {
        println!("0");
    }
}
