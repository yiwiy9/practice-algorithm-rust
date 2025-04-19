use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    let mut set = BTreeSet::new();

    let mut order = (0..s.len()).collect::<Vec<_>>();

    // next_permutation()
    permutohedron::heap_recursive(&mut order, |order| {
        let mut t = vec![];
        for i in 0..s.len() {
            t.push(s[order[i]]);
        }
        set.insert(t);
    });

    println!(
        "{}",
        set.iter().nth(k - 1).unwrap().iter().collect::<String>()
    );
}
