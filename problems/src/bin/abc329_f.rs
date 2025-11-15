use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
        queries: [(Usize1,Usize1); q],
    }

    let mut boxes = vec![BTreeSet::new(); n];
    for i in 0..n {
        boxes[i].insert(c[i]);
    }

    for &(a, b) in &queries {
        if boxes[a].len() > boxes[b].len() {
            boxes.swap(a, b);
        }

        let a_set = boxes[a].clone();
        for &c_i in &a_set {
            boxes[b].insert(c_i);
        }
        boxes[a] = BTreeSet::new();

        println!("{}", boxes[b].len());
    }
}
