use proconio::input;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    input! {
        (n, _m): (usize, usize),
        answers: [[u16]; n],
    }
    let mut result: HashSet<&u16, RandomState> = HashSet::from_iter(answers[0].iter());
    for v in answers[1..].iter() {
        let other = HashSet::from_iter(v.iter());
        result = result.intersection(&other).map(|i| *i).collect();
    }

    println!("{}", result.len());
}
