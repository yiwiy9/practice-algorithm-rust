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
        olr: [(usize,Usize1,Usize1); q],
    }

    let mut sum_segtree = Segtree::<Additive<usize>>::new(n);
    for i in 1..n {
        sum_segtree.set(i, if s[i] == s[i - 1] { 1 } else { 0 });
    }

    for &(op, l, r) in &olr {
        if op == 1 {
            if l > 0 {
                sum_segtree.set(l, sum_segtree.get(l) ^ 1);
            }
            if r < n - 1 {
                sum_segtree.set(r + 1, sum_segtree.get(r + 1) ^ 1);
            }
        } else {
            println!(
                "{}",
                if l == r || sum_segtree.prod(l + 1..=r) == 0 {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}
