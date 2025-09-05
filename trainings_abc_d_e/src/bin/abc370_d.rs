use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        rc: [(Usize1, Usize1); q],
    }

    let mut rows = vec![BTreeSet::from_iter(0..w); h];
    let mut cols = vec![BTreeSet::from_iter(0..h); w];

    for &(r, c) in &rc {
        if rows[r].contains(&c) {
            rows[r].remove(&c);
            cols[c].remove(&r);
            continue;
        }

        let left = rows[r].range(..c).next_back().cloned().unwrap_or(0);
        let right = rows[r].range(c..).next().cloned().unwrap_or(w - 1);
        rows[r].remove(&left);
        cols[left].remove(&r);
        rows[r].remove(&right);
        cols[right].remove(&r);

        let up = cols[c].range(..r).next_back().cloned().unwrap_or(0);
        let down = cols[c].range(r..).next().cloned().unwrap_or(h - 1);
        cols[c].remove(&up);
        rows[up].remove(&c);
        cols[c].remove(&down);
        rows[down].remove(&c);
    }

    println!("{}", rows.iter().map(|r| r.len()).sum::<usize>());
}
