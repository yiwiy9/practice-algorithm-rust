use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        tx: [(usize,usize); n],
    }

    let mut picked = vec![false; n];
    let mut having_x = vec![BTreeSet::new(); n + 1];

    for (i, &(t, x)) in tx.iter().enumerate() {
        match t {
            1 => {
                having_x[x].insert(i);
            }
            2 => {
                if having_x[x].is_empty() {
                    println!("{}", -1);
                    return;
                }

                let j = having_x[x].pop_last().unwrap();
                picked[j] = true;
            }
            _ => unreachable!(),
        }
    }

    let mut picked_ans = vec![];
    let mut k = 0;
    let mut k_max = 0;
    for (i, &(t, _)) in tx.iter().enumerate() {
        match t {
            1 => {
                if picked[i] {
                    k += 1;
                    k_max = k_max.max(k);
                    picked_ans.push("1");
                } else {
                    picked_ans.push("0");
                }
            }
            2 => {
                k -= 1;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", k_max);
    println!("{}", picked_ans.iter().join(" "));
}
