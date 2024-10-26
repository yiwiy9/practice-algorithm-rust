use ac_library::{Monoid, Segtree};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

struct DpIdx;
impl Monoid for DpIdx {
    type S = (usize, usize);
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.0 > b.0 {
            *a
        } else {
            *b
        }
    }
}

/// https://atcoder.jp/contests/abc369/editorial/10859
/// LISじゃないパターン
/// => dp[i] := (R0,C0) から (Ri,Ci) に移動するときに拾えるコインの枚数の最大値
/// (R,C)はソートされているとすると、
/// dp[i] = 1 + max{ dp[j] | (j < i, Cj <= Ci) }
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rc: [(Usize1, Usize1); n],
    }

    let mut new_n = n + 2;
    let mut new_rc = rc
        .into_iter()
        .chain([(0, 0), (h - 1, w - 1)].iter().cloned())
        .collect::<Vec<_>>();
    new_rc.sort();

    let mut segtree = Segtree::<DpIdx>::new(w);
    let mut dp = vec![0; new_n];
    let mut prev_idx_vec = vec![0; new_n];

    for i in 0..new_n {
        let (_, c) = new_rc[i];
        let prev = segtree.prod(0..=c);
        dp[i] = prev.0 + 1;
        prev_idx_vec[i] = prev.1;
        if segtree.get(c).0 < dp[i] {
            segtree.set(c, (dp[i], i));
        }
    }

    let mut moves = vec![];
    let mut idx = new_n - 1;
    while idx > 0 {
        let (to_r, to_c) = new_rc[idx];
        let (from_r, from_c) = new_rc[prev_idx_vec[idx]];
        for _ in from_r..to_r {
            moves.push('D');
        }
        for _ in from_c..to_c {
            moves.push('R');
        }
        idx = prev_idx_vec[idx];
    }
    moves.reverse();

    println!("{}", dp[new_n - 1] - 2);
    println!("{}", moves.iter().join(""));
}
