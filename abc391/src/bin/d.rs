use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;
use superslice::*;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        w: usize,
        xy: [(Usize1,Usize1); n],
        q: usize,
        ta: [(usize,Usize1); q],
    }

    let mut col_map = BTreeMap::new();
    for (i, &(x, y)) in xy.iter().enumerate() {
        col_map.entry(x).or_insert_with(Vec::new).push((y, i));
    }

    col_map.iter_mut().for_each(|(_, v)| {
        v.sort();
    });

    let mut max_time = vec![0; n];
    let mut max_time_cnt = vec![0; n];
    for (_, v) in col_map.iter() {
        for (i, &(y, _)) in v.iter().enumerate() {
            max_time[i] = max_time[i].max(y);
            max_time_cnt[i] += 1;
        }
    }

    for i in 0..n {
        if max_time_cnt[i] != w {
            max_time[i] = INF;
        }
    }

    for &(t, a) in &ta {
        let (x, y) = xy[a];
        let col = col_map.get(&x).unwrap();

        let col_idx = col.lower_bound(&(y, a));

        println!("{}", if max_time[col_idx] < t { "No" } else { "Yes" });
    }
}
