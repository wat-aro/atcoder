use proconio::input;

fn main() {
    input! {
        (x1, y1, x2, y2): (i64, i64, i64, i64)
    }
    let dx = x2 - x1;
    let dy = y2 - y1;
    let x3 = x2 - dy;
    let y3 = y2 + dx;
    let x4 = x1 - dy;
    let y4 = y1 + dx;

    println!("{} {} {} {}", x3, y3, x4, y4);
}
