use proconio::{input, marker::Usize1};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    input! {
        n: usize,
        x: [Usize1; n],
        c: [usize; n],
    }

    let mut hate = vec![0; n];
    for i in 0..n {
        hate[x[i]] += c[i];
    }

    let mut heap = BinaryHeap::new();
    for (i, &h) in hate.iter().enumerate() {
        heap.push(Reverse((h, i)));
    }

    let mut ans = 0;
    let mut used = HashSet::new();
    while let Some(Reverse((h, i))) = heap.pop() {
        if used.contains(&i) {
            continue;
        }

        ans += h;
        used.insert(i);

        hate[x[i]] -= c[i];
        heap.push(Reverse((hate[x[i]], x[i])));
    }

    println!("{}", ans);
}
