use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(u8,Usize1,Usize1); q],
    }

    let mut not_good_set = BTreeSet::new();
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            not_good_set.insert(i);
        }
    }

    for (t, l, r) in queries {
        if t == 1 {
            if l != 0 {
                if not_good_set.contains(&(l - 1)) {
                    not_good_set.remove(&(l - 1));
                } else {
                    not_good_set.insert(l - 1);
                }
            }
            if r != n - 1 {
                if not_good_set.contains(&r) {
                    not_good_set.remove(&r);
                } else {
                    not_good_set.insert(r);
                }
            }
        } else {
            println!(
                "{}",
                // Iterator の count は O(N) なので遅い
                // if not_good_set.range(l..r).count() == 0 {
                if not_good_set.range(l..r).next().is_none() {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}
