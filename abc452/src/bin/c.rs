use std::collections::{HashMap, HashSet};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        ab: [(usize,Usize1); n],
        m: usize,
        s: [Chars; m],
    }

    let mut map = HashMap::new();
    for s_j in &s {
        let a = s_j.len();
        for (b, &c) in s_j.iter().enumerate() {
            map.entry((a, b)).or_insert(HashSet::new()).insert(c);
        }
    }

    for s_j in &s {
        let mut ok = true;
        if s_j.len() != n {
            println!("No");
            continue;
        }
        for (i, &c) in s_j.iter().enumerate() {
            let (a, b) = ab[i];
            if let Some(set) = map.get(&(a, b)) {
                if !set.contains(&c) {
                    ok = false;
                    break;
                }
            } else {
                ok = false;
                break;
            }
        }
        println!("{}", if ok { "Yes" } else { "No" });
    }
}
