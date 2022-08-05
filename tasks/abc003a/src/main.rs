use core::panic;

use proconio::input;

#[derive(Debug)]
struct Travel {
    n: i64,
    w: i64,
    s: i64,
    e: i64,
}

impl Travel {
    fn new() -> Self {
        Travel {
            n: 0,
            w: 0,
            s: 0,
            e: 0,
        }
    }

    fn calculate(&self) -> bool {
        let ns = (self.n > 0 && self.s > 0) || (self.n == 0 && self.s == 0);
        let we = (self.w > 0 && self.e > 0) || (self.w == 0 && self.e == 0);

        ns && we
    }
}

fn main() {
    input! {
        s: String
    }
    let mut travel = Travel::new();

    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();

        match c {
            'N' => travel.n += 1 as i64,
            'W' => travel.w += 1 as i64,
            'S' => travel.s += 1 as i64,
            'E' => travel.e += 1 as i64,
            _ => panic!("Unhandled character: {}", c),
        }
    }

    if travel.calculate() {
        println!("Yes");
    } else {
        println!("No");
    }
}
