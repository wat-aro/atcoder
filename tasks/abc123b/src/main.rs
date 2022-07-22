use proconio::input;

fn main() {
    input! {
        v: [u16; 5],
    }
    let min = v.iter().enumerate().fold((0, 9), |acc, (index, value)| {
        let one = value % 10;
        if one == 0 {
            acc
        } else {
            if acc.1 < one {
                acc
            } else {
                (index, one)
            }
        }
    });
    let answer: u16 = v
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let one = value % 10;
            if one == 0 {
                *value
            } else if index == min.0 {
                *value
            } else {
                value + 10 - one
            }
        })
        .sum::<u16>();
    println!("{}", answer);
}
