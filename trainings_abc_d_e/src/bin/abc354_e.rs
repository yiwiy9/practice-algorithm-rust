use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
    }

    println!(
        "{}",
        if rec(&ab, &mut vec![false; n], &mut HashMap::new()) == 0 {
            "Aoki"
        } else {
            "Takahashi"
        }
    );
}

fn rec(ab: &[(usize, usize)], used: &mut Vec<bool>, memo: &mut HashMap<Vec<bool>, usize>) -> usize {
    if let Some(&g) = memo.get(used) {
        return g;
    }

    let mut grundy = HashSet::new();

    for i in 0..ab.len() {
        if used[i] {
            continue;
        }
        for j in i + 1..ab.len() {
            if used[j] {
                continue;
            }
            if ab[i].0 == ab[j].0 || ab[i].1 == ab[j].1 {
                used[i] = true;
                used[j] = true;
                grundy.insert(rec(ab, used, memo));
                used[i] = false;
                used[j] = false;
            }
        }
    }

    let mut mex = 0;
    while grundy.contains(&mex) {
        mex += 1;
    }

    memo.insert(used.clone(), mex);
    mex
}
