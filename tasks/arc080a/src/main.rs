use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut odd = 0;
    let mut four = 0;
    let mut even = 0;

    for ai in a {
        if ai % 4 == 0 {
            four += 1;
        } else if ai % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }
    if odd > four + 1 {
        println!("No");
    } else if even == 0 {
        println!("Yes");
    } else if odd <= four {
        println!("Yes")
    } else {
        println!("No");
    }
}
