use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    for i in (0..=n).rev() {
        let mut set = BTreeSet::new();
        for j in 0..i {
            set.insert(a[j]);
        }
        if set.len() < m {
            println!("{}", n - i);
            return;
        }
    }
}
