use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1,Usize1,usize); m],
    }

    let mut road = HashMap::new();
    for (a, b, c) in abc {
        road.entry((a, b))
            .and_modify(|e| {
                if *e < c {
                    *e = c;
                }
            })
            .or_insert(c);

        road.entry((b, a))
            .and_modify(|e| {
                if *e < c {
                    *e = c;
                }
            })
            .or_insert(c);
    }

    let mut order = (0..n).collect::<Vec<_>>();

    let mut ans = 0;
    // next_permutation()
    permutohedron::heap_recursive(&mut order, |order| {
        let mut cur = 0;
        for i in 0..n - 1 {
            let a = order[i];
            let b = order[i + 1];
            if let Some(&c) = road.get(&(a, b)) {
                cur += c;
            } else {
                break;
            }
        }
        ans = ans.max(cur);
    });

    println!("{}", ans);
}
