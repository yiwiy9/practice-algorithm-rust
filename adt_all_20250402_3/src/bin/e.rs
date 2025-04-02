use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut edges = HashMap::new();
    for &(a, b, c) in &abc {
        edges.insert((a, b), c);
        edges.insert((b, a), c);
    }

    let mut order = (0..n).collect::<Vec<_>>();

    let mut ans = 0;

    // next_permutation()
    permutohedron::heap_recursive(&mut order, |order| {
        let mut cnt = 0;

        for i in 1..n {
            if !edges.contains_key(&(order[i - 1], order[i])) {
                break;
            }
            cnt += edges[&(order[i - 1], order[i])];
        }

        ans = ans.max(cnt);
    });

    println!("{}", ans);
}
