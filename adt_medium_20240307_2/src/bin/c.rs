use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut edges = BTreeSet::new();
    for (u, v) in uv {
        edges.insert((u, v));
        edges.insert((v, u));
    }

    let mut ans = 0;
    for a in 0..100 {
        for b in (a + 1)..100 {
            for c in (b + 1)..100 {
                if edges.contains(&(a, b)) && edges.contains(&(b, c)) && edges.contains(&(c, a)) {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
