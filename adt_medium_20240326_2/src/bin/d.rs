use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut friends = vec![BTreeSet::new(); n];
    for _ in 0..m {
        input! {
            k: usize,
            x: [Usize1; k],
        }
        for &x_i in &x {
            for &x_j in &x {
                friends[x_i].insert(x_j);
            }
        }
    }

    println!(
        "{}",
        if friends.iter().all(|f| f.len() == n) {
            "Yes"
        } else {
            "No"
        }
    );
}
