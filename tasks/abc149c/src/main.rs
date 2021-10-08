use proconio::input;

fn main() {
    input! {
        mut x: u32,
    }
    let mut sieve: Vec<u32> = Vec::new();

    if x == 2 {
        println!("2");
        return;
    }

    let sqrt = (x as f64).sqrt().floor() as u32;

    (2..sqrt).for_each(|i| {
        if sieve.iter().all(|&j| i != j) {
            sieve.push(i)
        }
    });

    loop {
        if sieve.iter().any(|i| x % i == 0) {
            x = x + 1;
        } else {
            break;
        }
    }

    println!("{}", x);
}
