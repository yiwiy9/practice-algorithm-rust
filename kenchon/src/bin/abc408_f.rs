use ac_library::{Max, Segtree};
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        d: usize,
        r: usize,
        h: [usize; n],
    }

    let mut num_idx_map = HashMap::new();
    for (i, &h_i) in h.iter().enumerate() {
        num_idx_map.insert(h_i, i);
    }

    let mut num_vec = vec![0; n + 1];
    let mut max_segtree = Segtree::<Max<usize>>::new(n);

    let mut ans: usize = 0;
    for num in 1..=n {
        if num > d {
            let idx_before_d = *num_idx_map.get(&(num - d)).unwrap();
            max_segtree.set(idx_before_d, num_vec[num - d]);
        }

        let idx = *num_idx_map.get(&num).unwrap();
        let left = idx.saturating_sub(r);
        let right = (idx + r).min(n - 1);
        let range_max = max_segtree.prod(left..=right);

        num_vec[num] = range_max + 1;
        ans = ans.max(num_vec[num]);
    }

    println!("{}", ans - 1);
}
