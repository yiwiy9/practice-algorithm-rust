use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [Usize1; n],
    }

    let mut see = vec![0; n];
    for i in 0..n {
        see[q[i]] = i;
    }

    let mut ans = vec![];
    for i in 0..n {
        let p_idx = see[i];
        let q_idx = p[p_idx];
        ans.push(q[q_idx] + 1);
    }

    println!("{}", ans.iter().join(" "));
}
