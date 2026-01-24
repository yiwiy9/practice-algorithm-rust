use ac_library::{Max, Segtree};
use proconio::input;

const MAX: usize = 300_010;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut max_segtree = Segtree::<Max<usize>>::new(MAX);
    for &a_i in &a {
        let l = a_i.saturating_sub(k);
        let r = (a_i + k).min(MAX - 1);
        let max_cnt = if l < r {
            max_segtree.prod(l..=r)
        } else {
            max_segtree.get(a_i)
        };
        max_segtree.set(a_i, max_cnt + 1);
    }

    println!("{}", max_segtree.all_prod());
}
