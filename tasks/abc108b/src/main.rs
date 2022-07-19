use proconio::input;

fn main() {
    input! {
        (x1, y1, x2, y2): (i64, i64, i64, i64)
    }
    let (x3, y3) = (x2 + y1 - y2, y2 + x2 - x1);
    let (x4, y4) = (x1 + y1 - y2, y1 + x2 - x1);
    println!("{} {} {} {}", x3, y3, x4, y4);
}
