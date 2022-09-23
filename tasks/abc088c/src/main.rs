use proconio::input;

fn main() {
    input! {
        c: [[i16; 3]; 3]
    }
    for a1 in 0..=*c[0].iter().min().unwrap() {
        for a2 in 0..=*c[1].iter().min().unwrap() {
            for a3 in 0..=*c[2].iter().min().unwrap() {
                for b1 in 0..=((c[0][0] - a1).min(c[1][0] - a2).min(c[2][0] - a3)) {
                    for b2 in 0..=(c[0][1] - a1).min(c[1][1] - a2).min(c[2][1] - a3) {
                        for b3 in 0..=(c[0][2] - a1).min(c[1][2] - a2).min(c[2][2] - a3) {
                            if a1 + b1 == c[0][0]
                                && a1 + b2 == c[0][1]
                                && a1 + b3 == c[0][2]
                                && a2 + b1 == c[1][0]
                                && a2 + b2 == c[1][1]
                                && a2 + b3 == c[1][2]
                                && a3 + b1 == c[2][0]
                                && a3 + b2 == c[2][1]
                                && a3 + b3 == c[2][2]
                            {
                                println!("Yes");
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
