use itertools::Itertools as _;
use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        tx: [(u8, Usize1); n],
    }

    let mut monsters = HashMap::new();
    let mut ans = vec![2; n];
    for (i, &(t, x)) in tx.iter().enumerate().rev() {
        match t {
            1 => {
                if monsters.contains_key(&x) {
                    if monsters[&x] == 1 {
                        monsters.remove(&x);
                    } else {
                        monsters.entry(x).and_modify(|cur| *cur -= 1);
                    }
                    ans[i] = 1;
                } else {
                    ans[i] = 0;
                }
            }
            2 => {
                monsters.entry(x).and_modify(|cur| *cur += 1).or_insert(1);
            }
            _ => unreachable!(),
        }
    }

    if !monsters.is_empty() {
        println!("-1");
        return;
    }

    let mut max_k = 0;
    let mut cur_k = 0;
    for &a in &ans {
        if a == 1 {
            cur_k += 1;
        } else if a == 2 {
            cur_k -= 1;
        }
        max_k = max_k.max(cur_k);
    }

    println!("{}", max_k);
    println!("{}", ans.iter().filter(|&&a| a != 2).join(" "));
}
