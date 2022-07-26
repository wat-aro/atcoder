use proconio::input;

fn solve(a: String, b: String) -> String {
    if a.len() > b.len() {
        return "GREATER".to_string();
    } else if a.len() < b.len() {
        return "LESS".to_string();
    } else {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => "LESS".to_string(),
            std::cmp::Ordering::Equal => "EQUAL".to_string(),
            std::cmp::Ordering::Greater => "GREATER".to_string(),
        }
    }
}

fn main() {
    input! {
        a: String,
        b: String
    }
    let answer = solve(a, b);

    println!("{}", answer);
}
