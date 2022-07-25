use proconio::input;

struct Lucus {
    current: u64,
    next: u64,
}

impl Iterator for Lucus {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;

        self.current = self.next;
        self.next = new_next;

        Some(self.current)
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut lucus = Lucus {
        current: 2,
        next: 1,
    };
    println!("{}", lucus.nth(n - 1).unwrap());
}
