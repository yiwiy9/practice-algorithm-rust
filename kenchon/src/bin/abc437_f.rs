use ac_library::{Max, Min, Segtree};
use proconio::{input, marker::Usize1};

/// https://qiita.com/yuu_kyopro/items/8b11ad374227b3e86fb1
/// マンハッタン距離45度回転
/// |Xi - Xj| + |Yi - Yj| = max(|Zi - Zj|, |Wi - Wj|)
/// ただし、Zi = Xi + Yi, Wi = Xi - Yi
/// => O(N^2)かかるところを、iごとに最大値を見るだけで良くなる
fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
    }

    let mut z_min_segtree = Segtree::<Min<i64>>::new(n);
    let mut z_max_segtree = Segtree::<Max<i64>>::new(n);
    let mut w_min_segtree = Segtree::<Min<i64>>::new(n);
    let mut w_max_segtree = Segtree::<Max<i64>>::new(n);
    for (i, &(x, y)) in xy.iter().enumerate() {
        z_min_segtree.set(i, x + y);
        z_max_segtree.set(i, x + y);
        w_min_segtree.set(i, x - y);
        w_max_segtree.set(i, x - y);
    }

    for _ in 0..q {
        input! {
            op: usize,
        }
        match op {
            1 => {
                input! {
                    i: Usize1,
                    x: i64,
                    y: i64,
                }
                z_min_segtree.set(i, x + y);
                z_max_segtree.set(i, x + y);
                w_min_segtree.set(i, x - y);
                w_max_segtree.set(i, x - y);
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                    x: i64,
                    y: i64,
                }

                let z = x + y;
                let w = x - y;

                let z_min = z_min_segtree.prod(l..=r);
                let z_max = z_max_segtree.prod(l..=r);
                let w_min = w_min_segtree.prod(l..=r);
                let w_max = w_max_segtree.prod(l..=r);

                println!(
                    "{}",
                    (z - z_min)
                        .abs()
                        .max((z - z_max).abs())
                        .max((w - w_min).abs())
                        .max((w - w_max).abs())
                );
            }
            _ => unreachable!(),
        }
    }
}
