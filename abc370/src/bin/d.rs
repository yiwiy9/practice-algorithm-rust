use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        rc: [(Usize1,Usize1); q],
    }

    let mut rows = vec![];
    for i in 0..h {
        let mut set = BTreeSet::new();
        for j in 0..w {
            set.insert(j);
        }
        rows.push(set.clone());
    }
    let mut cols = vec![];
    for i in 0..w {
        let mut set = BTreeSet::new();
        for j in 0..h {
            set.insert(j);
        }
        cols.push(set.clone());
    }

    for &(r, c) in &rc {
        if rows[r].contains(&c) {
            rows[r].remove(&c);
            cols[c].remove(&r);
            continue;
        }

        if let Some(c_left) = rows[r].range(..c).next_back().copied() {
            rows[r].remove(&c_left);
            cols[c_left].remove(&r);
        }

        if let Some(c_right) = rows[r].range(c..).next().copied() {
            rows[r].remove(&c_right);
            cols[c_right].remove(&r);
        }

        if let Some(r_up) = cols[c].range(..r).next_back().copied() {
            cols[c].remove(&r_up);
            rows[r_up].remove(&c);
        }

        if let Some(r_down) = cols[c].range(r..).next().copied() {
            cols[c].remove(&r_down);
            rows[r_down].remove(&c);
        }
    }

    println!("{}", rows.iter().map(|row| row.len()).sum::<usize>());
}
