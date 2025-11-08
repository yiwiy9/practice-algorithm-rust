use proconio::{input, marker::Usize1};
use std::collections::{BTreeSet, HashMap};

fn main() {
    input! {
        h: usize,
        w: usize,
        r_s: Usize1,
        c_s: Usize1,
        n: usize,
        rc: [(Usize1,Usize1); n],
        q: usize,
        dl: [(char,usize); q],
    }

    let mut row_set_map = HashMap::new();
    let mut column_set_map = HashMap::new();
    for &(r, c) in &rc {
        row_set_map.entry(r).or_insert(BTreeSet::new()).insert(c);
        column_set_map.entry(c).or_insert(BTreeSet::new()).insert(r);
    }

    let mut r = r_s;
    let mut c = c_s;
    for &(d, l) in &dl {
        match d {
            'L' => {
                if let Some(set) = row_set_map.get(&r) {
                    if let Some(&wall) = set.range(..c).next_back() {
                        c = if wall + l >= c { wall + 1 } else { c - l };
                    } else {
                        c = c.saturating_sub(l);
                    }
                } else {
                    c = c.saturating_sub(l);
                }
                println!("{} {}", r + 1, c + 1)
            }
            'R' => {
                if let Some(set) = row_set_map.get(&r) {
                    if let Some(&wall) = set.range(c + 1..).next() {
                        c = if c + l >= wall { wall - 1 } else { c + l };
                    } else {
                        c = (c + l).min(w - 1);
                    }
                } else {
                    c = (c + l).min(w - 1);
                }
                println!("{} {}", r + 1, c + 1)
            }
            'U' => {
                if let Some(set) = column_set_map.get(&c) {
                    if let Some(&wall) = set.range(..r).next_back() {
                        r = if wall + l >= r { wall + 1 } else { r - l };
                    } else {
                        r = r.saturating_sub(l);
                    }
                } else {
                    r = r.saturating_sub(l);
                }
                println!("{} {}", r + 1, c + 1)
            }
            'D' => {
                if let Some(set) = column_set_map.get(&c) {
                    if let Some(&wall) = set.range(r + 1..).next() {
                        r = if r + l >= wall { wall - 1 } else { r + l };
                    } else {
                        r = (r + l).min(h - 1);
                    }
                } else {
                    r = (r + l).min(h - 1);
                }
                println!("{} {}", r + 1, c + 1)
            }
            _ => unreachable!(),
        }
    }
}
