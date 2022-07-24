use proconio::input;

fn main() {
    input! {
        s: String
    }

    let count0 = s.chars().enumerate().fold(0, |acc, (index, c)| {
        if index % 2 == 0 && c == '0' {
            acc + 1
        } else if index % 2 == 1 && c == '1' {
            acc + 1
        } else {
            acc
        }
    });

    let count1 = s.chars().enumerate().fold(0, |acc, (index, c)| {
        if index % 2 == 0 && c == '1' {
            acc + 1
        } else if index % 2 == 1 && c == '0' {
            acc + 1
        } else {
            acc
        }
    });

    println!("{:?}", count0.min(count1));
}
