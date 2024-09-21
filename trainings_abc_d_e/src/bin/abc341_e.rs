// use proconio::{
//     input,
//     marker::{Chars, Usize1},
// };
// use std::collections::BTreeSet;

// fn main() {
//     input! {
//         n: usize,
//         q: usize,
//         s: Chars,
//         queries: [(u8,Usize1,Usize1); q],
//     }

//     let mut not_good_set = BTreeSet::new();
//     for i in 0..n - 1 {
//         if s[i] == s[i + 1] {
//             not_good_set.insert(i);
//         }
//     }

//     for (t, l, r) in queries {
//         if t == 1 {
//             if l != 0 {
//                 if not_good_set.contains(&(l - 1)) {
//                     not_good_set.remove(&(l - 1));
//                 } else {
//                     not_good_set.insert(l - 1);
//                 }
//             }
//             if r != n - 1 {
//                 if not_good_set.contains(&r) {
//                     not_good_set.remove(&r);
//                 } else {
//                     not_good_set.insert(r);
//                 }
//             }
//         } else {
//             println!(
//                 "{}",
//                 // Iterator の count は O(N) なので遅い
//                 // if not_good_set.range(l..r).count() == 0 {
//                 if not_good_set.range(l..r).next().is_none() {
//                     "Yes"
//                 } else {
//                     "No"
//                 }
//             );
//         }
//     }
// }

/**
 * セグメント木を使った解法
 * https://github.com/rust-lang-ja/ac-library-rs/blob/master/examples/practice2_j_segment_tree.rs
 */
use ac_library::{Additive, Segtree};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(u8,Usize1,Usize1); q],
    }

    // 良い間なら 1, そうでない区間なら 0
    let mut segtree = Segtree::<Additive<usize>>::new(n - 1);
    for i in 0..n - 1 {
        if s[i] != s[i + 1] {
            segtree.set(i, 1);
        }
    }

    for (t, l, r) in queries {
        if t == 1 {
            if l != 0 {
                segtree.set(l - 1, 1 - segtree.get(l - 1));
            }
            if r != n - 1 {
                segtree.set(r, 1 - segtree.get(r));
            }
        } else {
            println!(
                "{}",
                if segtree.prod(l..r) == r - l {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}
