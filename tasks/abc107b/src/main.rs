use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _w: usize,
        a: [Chars; h],
    }
    let a = a
        .iter()
        .filter(|l| !l.iter().all(|c| *c == '.'))
        .collect::<Vec<_>>();
    let transposed = transpose(a);
    let v = transposed
        .iter()
        .filter(|l| !l.iter().all(|c| *c == '.'))
        .collect::<Vec<_>>();
    let t = transpose(v);
    println!(
        "{}",
        t.iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn transpose<T>(v: Vec<&Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
