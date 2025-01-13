use ac_library::{Monoid, Segtree};
use proconio::{input, marker::Usize1};
use superslice::*;

struct LenDist;
impl Monoid for LenDist {
    type S = (usize, usize);
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(l: &Self::S, r: &Self::S) -> Self::S {
        let (l_len, l_dist) = *l;
        let (r_len, r_dist) = *r;
        (l_len + r_len, l_dist.max(r_dist))
    }
}

/// https://atcoder.jp/contests/abc388/tasks/abc388_g
/// https://atcoder.jp/contests/abc388/editorial/11904
/// クエリ(L,R) に対する答えは L + K − 1 + max{ Bk - k, k } <= R となる最大の K である。
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(Usize1, Usize1); q],
    }

    let mut segtree = Segtree::<LenDist>::new(n);
    for (i, &a_i) in a.iter().enumerate() {
        let b_i = a.lower_bound(&(2 * a_i));
        segtree.set(i, (1, b_i - i));
    }

    for &(l, r) in &lr {
        // max_right は l から binary_operation() を適用していって、条件を満たす最大の r を返す
        let k = segtree.max_right(l, |&(cur_len, cur_dist)| {
            l + cur_len + cur_dist.max(cur_len) <= r + 1
        });
        println!("{}", k - l);
    }
}
