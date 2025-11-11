use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut set = HashSet::new();
    for &(a, b) in &ab {
        set.insert((a, b));
        if a + 2 < n && b + 1 < n {
            set.insert((a + 2, b + 1));
        }
        if a + 1 < n && b + 2 < n {
            set.insert((a + 1, b + 2));
        }
        if a >= 1 && b + 2 < n {
            set.insert((a - 1, b + 2));
        }
        if a >= 2 && b + 1 < n {
            set.insert((a - 2, b + 1));
        }
        if a >= 2 && b >= 1 {
            set.insert((a - 2, b - 1));
        }
        if a >= 1 && b >= 2 {
            set.insert((a - 1, b - 2));
        }
        if a + 1 < n && b >= 2 {
            set.insert((a + 1, b - 2));
        }
        if a + 2 < n && b >= 1 {
            set.insert((a + 2, b - 1));
        }
    }

    println!("{}", n * n - set.len());
}
