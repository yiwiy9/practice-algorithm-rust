use proconio::input;
use std::collections::{BTreeMap, BinaryHeap};

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [usize; n],
        l: [usize; m],
        d: [usize; m],
    }

    let mut p_map = BTreeMap::new();
    for &pi in &p {
        *p_map.entry(pi).or_insert(0) += 1;
    }

    let mut dl_heap = BinaryHeap::new();
    for i in 0..m {
        dl_heap.push((d[i], l[i]));
    }

    let mut ans = 0;
    while let Some((d_i, l_i)) = dl_heap.pop() {
        if let Some((&p_num, &p_cnt)) = p_map.range(l_i..).next() {
            ans += p_num - d_i;
            if p_cnt == 1 {
                p_map.remove(&p_num);
            } else {
                p_map.insert(p_num, p_cnt - 1);
            }
        }
    }

    for (&p_num, &p_cnt) in &p_map {
        ans += p_num * p_cnt;
    }

    println!("{}", ans);
}
