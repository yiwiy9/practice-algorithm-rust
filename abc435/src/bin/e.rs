use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;
use superslice::*;

/**
 * 区間変更・区間和取得
 * 区間幅が必要なので値を構造体で持ちます
 *
 * チートシート
 *  - https://betrue12.hateblo.jp/entry/2020/09/23/005940
 * 使い方の詳細
 *  - https://betrue12.hateblo.jp/entry/2020/09/22/194541
 */
#[derive(Clone, Copy)]
struct S {
    value: usize,
    size: usize,
}
struct M;
impl Monoid for M {
    type S = S;

    fn identity() -> Self::S {
        S { value: 0, size: 0 }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        S {
            value: a.value + b.value,
            size: a.size + b.size,
        }
    }
}
struct F;
impl MapMonoid for F {
    type M = M;
    type F = Option<usize>;

    fn identity_map() -> Self::F {
        // lazyの値として取り得ないような値を擬似的に恒等写像として扱う
        None
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        match f {
            Some(v) => S {
                value: v * x.size,
                size: x.size,
            },
            None => *x,
        }
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f.or(*g)
    }
}

/// https://atcoder.jp/contests/abc435/tasks/abc435_e
/// https://atcoder.jp/contests/abc435/editorial/14743
/// => クエリ先読みができるとき、動的セグ木を使わずとも座標圧縮 + セグ木でできる場合が多いです（この考察の流れは頻繁に出会います）。
///
/// ポイントは、
/// 座標圧縮した後の点をセグ木に対応させるのではなく、
/// 座標圧縮した後の点とその隣の点の区間を1つの点とみなしてセグ木に対応させる
///
/// なぜこのようにしたいのか？
/// 座標圧縮した後の点をそのままセグ木に対応させると、
/// 圧縮した区間の値（セグ木で管理したい値）を左の点 or 右の点のどちらに乗せるかという問題が生じる
/// 圧縮後の区間を点とみなす事で意味通りの値を設定できる
/// ただし、区間は左端は含んで右端は含まないものとする
///
/// => あとはクエリ通りにシミュレーションするだけ
fn main() {
    input! {
        n: usize,
        q: usize,
        mut lr: [(usize,usize); q],
    }

    // 右端含まない
    lr = lr.iter().map(|&(l, r)| (l, r + 1)).collect::<Vec<_>>();

    let mut b = vec![];
    for &(l, r) in &lr {
        b.push(l);
        b.push(r);
    }

    // 区間の左端と右端を管理下に置く
    b.push(1); // 左端含む
    b.push(n + 1); // 右端含まない

    b.sort();
    b.dedup();

    // 座標圧縮後の点ではなく、区間をセグ木上の点とする
    let mut points = vec![];
    for i in 0..b.len() - 1 {
        let value = b[i + 1] - b[i];
        points.push(S { value, size: 1 });
    }

    let mut lazy_segtree = LazySegtree::<F>::from(points);
    for &(l, r) in &lr {
        let l_i = b.lower_bound(&l);
        let r_i = b.lower_bound(&r);

        lazy_segtree.apply_range(l_i..r_i, Some(0));
        println!("{}", lazy_segtree.all_prod().value);
    }
}
