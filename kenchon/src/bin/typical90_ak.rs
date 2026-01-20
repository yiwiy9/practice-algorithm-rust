use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        w: usize,
        n: usize,
        lrv: [(usize, usize, i64); n],
    }

    let mut max_segtree = Segtree::<Max<i64>>::from_iter(std::iter::repeat_n(-1, w + 1));
    max_segtree.set(0, 0);
    for &(l, r, v) in &lrv {
        let mut next_max_segtree = Segtree::<Max<i64>>::from_iter(std::iter::repeat_n(-1, w + 1));
        next_max_segtree.set(0, 0);

        for j in 0..=w {
            next_max_segtree.set(j, next_max_segtree.get(j).max(max_segtree.get(j)));

            let before_l = j.saturating_sub(r);
            let before_r = j.saturating_sub(l);
            let before_max = max_segtree.prod(before_l..=before_r);
            if j >= l && before_max != -1 {
                let next_v = before_max + v;
                next_max_segtree.set(j, next_max_segtree.get(j).max(next_v));
            }
        }

        max_segtree = next_max_segtree;
    }

    println!("{}", max_segtree.get(w));
}
