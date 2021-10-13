use proconio::input;

fn main() {
    input! {
        a: [[u8; 3]; 3],
        n: u8,
        b: [u8; n]
    }

    let v: Vec<Vec<bool>> = a
        .iter()
        .map(move |ai| {
            ai.iter()
                .map(|aij| b.iter().any(|&bi| *aij == bi))
                .collect()
        })
        .collect();

    let answer: bool = v.iter().any(|vi| vi.iter().all(|&vij| vij))
        || transpose(&v).iter().any(|vi| vi.iter().all(|&vij| vij))
        || (v[0][0] && v[1][1] && v[2][2])
        || (v[0][2] && v[1][1] && v[2][0]);

    println!("{}", if answer { "Yes" } else { "No" });
}

fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
