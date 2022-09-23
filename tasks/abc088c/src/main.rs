use proconio::input;

fn main() {
    input! {
        c: [[i16; 3]; 3]
    }
    for a1 in 0..=*c[0].iter().min().unwrap() {
        let b1 = c[0][0] - a1;
        let b2 = c[0][1] - a1;
        let b3 = c[0][2] - a1;
        let a2 = c[1][0] - b1;
        let a3 = c[2][0] - b1;

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
    println!("No");
}
