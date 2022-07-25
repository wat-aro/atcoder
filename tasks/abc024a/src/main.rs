use proconio::input;

fn main() {
    input! {
        (a, b, _c, k): (i64, i64, i64, i64)
    }
    let answer;
    if k % 2 == 0 {
        answer = a.checked_sub(b);
    } else {
        answer = b.checked_sub(a);
    }

    if let Some(i) = answer {
        println!("{}", i);
    } else {
        println!("Unfair");
    }
}
