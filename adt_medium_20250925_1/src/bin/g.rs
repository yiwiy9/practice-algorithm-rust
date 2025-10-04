use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    }

    let mut q = vec![0; n];
    for (i, &p_i) in p.iter().enumerate() {
        q[p_i] = i;
    }

    let mut set = BTreeSet::new();
    for &q_i in q.iter().take(k) {
        set.insert(q_i);
    }

    let mut ans = set.last().unwrap() - set.first().unwrap();
    for i in k..n {
        set.remove(&q[i - k]);
        set.insert(q[i]);
        ans = ans.min(set.last().unwrap() - set.first().unwrap());
    }

    println!("{}", ans);
}
