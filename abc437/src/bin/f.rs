use ac_library::{Max, Min, Segtree};
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc437/tasks/abc437_f
/// https://atcoder.jp/contests/abc437/editorial/14891
///
/// マンハッタン距離の 45 度回転
/// （https://qiita.com/yuu_kyopro/items/8b11ad374227b3e86fb1）
///
/// |x - Xi| + |y - Yi|
/// = max{ |(x+y) - (Xi+Yi)|, |(x-y) - (Xi-Yi)| }
/// = max{ |u - Ui|, |v - Vi| }
///
/// => Ui, Vi の最小値 or 最大値のみが答えに影響することがわかる
/// => Ui, Vi は事前に決められるので、それぞれを独立で考えると、見かけ上は1次元となり、セグ木に乗る
fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64,i64); n],
    }

    let mut min_u_segtree = Segtree::<Min<i64>>::new(n);
    let mut max_u_segtree = Segtree::<Max<i64>>::new(n);
    let mut min_v_segtree = Segtree::<Min<i64>>::new(n);
    let mut max_v_segtree = Segtree::<Max<i64>>::new(n);
    for (i, &(x, y)) in xy.iter().enumerate() {
        min_u_segtree.set(i, x + y);
        max_u_segtree.set(i, x + y);
        min_v_segtree.set(i, x - y);
        max_v_segtree.set(i, x - y);
    }

    for _ in 0..q {
        input! {
            op: usize,
        }

        if op == 1 {
            input! {
                i: Usize1,
                x: i64,
                y: i64,
            }
            min_u_segtree.set(i, x + y);
            max_u_segtree.set(i, x + y);
            min_v_segtree.set(i, x - y);
            max_v_segtree.set(i, x - y);
        } else {
            input! {
                l: Usize1,
                r: Usize1,
                x: i64,
                y: i64,
            }

            let mut ans = 0;
            ans = ans.max((x + y - min_u_segtree.prod(l..=r)).abs());
            ans = ans.max((x + y - max_u_segtree.prod(l..=r)).abs());
            ans = ans.max((x - y - min_v_segtree.prod(l..=r)).abs());
            ans = ans.max((x - y - max_v_segtree.prod(l..=r)).abs());

            println!("{}", ans);
        }
    }
}
