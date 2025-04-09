use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    }

    let mut indexes = vec![0; n];
    for i in 0..n {
        indexes[p[i]] = i;
    }

    let mut set = BTreeSet::new();
    for i in 0..k {
        set.insert(indexes[i]);
    }

    let mut ans = INF;
    for i in 0..n - k + 1 {
        if i > 0 {
            set.remove(&indexes[i - 1]);
            set.insert(indexes[i + k - 1]);
        }
        let min = *set.iter().next().unwrap();
        let max = *set.iter().next_back().unwrap();
        ans = ans.min(max - min);
    }

    println!("{}", ans);
}
