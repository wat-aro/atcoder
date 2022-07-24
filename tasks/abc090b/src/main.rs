use proconio::input;

fn main() {
    input! {
        (a, b): (u32, u32)
    }
    let mut count = 0;
    for i in a..=b {
        let i1 = i % 10;
        let i10000 = i / 10000;
        let i10 = (i / 10) % 10;
        let i1000 = (i / 1000) % 10;
        if i1 == i10000 && i10 == i1000 {
            count += 1;
        }
    }
    println!("{}", count);
}
