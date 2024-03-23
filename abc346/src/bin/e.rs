use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;
use superslice::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        tax: [(usize, Usize1, usize); m],
    }

    let mut row = vec![(0_usize, 0_usize); h];
    let mut column = vec![(0_usize, 0_usize); w];

    for (i, &(t, a, x)) in tax.iter().enumerate() {
        if t == 1 {
            row[a] = (i + 1, x);
        } else {
            column[a] = (i + 1, x);
        }
    }

    let mut row_i_vec = row.iter().map(|&(i, _)| i).collect::<Vec<_>>();
    let mut column_i_vec = column.iter().map(|&(i, _)| i).collect::<Vec<_>>();
    row_i_vec.sort();
    column_i_vec.sort();

    let mut map = BTreeMap::new();
    for (i, x) in row {
        let cnt = column_i_vec.upper_bound(&i);
        if cnt == 0 {
            continue;
        }
        map.entry(x).and_modify(|cur| *cur += cnt).or_insert(cnt);
    }
    for (i, x) in column {
        let cnt = row_i_vec.upper_bound(&i);
        if cnt == 0 || i == 0 {
            continue;
        }
        map.entry(x).and_modify(|cur| *cur += cnt).or_insert(cnt);
    }

    println!("{}", map.len());
    for (x, &cnt) in map.iter() {
        println!("{} {}", x, cnt);
    }
}
