use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [Usize1; n],
    }

    let mut r = vec![0; n];
    for (i, &q_i) in q.iter().enumerate() {
        r[q_i] = i;
    }

    let mut ans = vec![0; n];
    for (q_i, &i) in r.iter().enumerate() {
        ans[q_i] = q[p[i]] + 1;
    }

    println!("{}", ans.iter().join(" "));
}
