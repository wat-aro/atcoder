use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u64; n]
    }
    if n == 1 {
        println!("Yes");
        return;
    }

    let h = h.iter().rev();
    let mut current = std::u64::MAX;

    for x in h {
        if *x <= current {
            current = *x;
        } else if x - 1 <= current {
            current = x - 1;
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
