// use itertools::Itertools;
// use proconio::input;

// /**
//  * https://atcoder.jp/contests/abc372/tasks/abc372_d
//  * https://atcoder.jp/contests/abc372/editorial/10975
//  *
//  * 各 i に対して条件を満たす j を昇順に並べたとき、Hj1 < Hj2 < ... が成り立つ
//  * このことに気づけば、i を後ろから見ていって、Hj1, Hj2, ... と Hi が入るところまで遡ればよい
//  */
// fn main() {
//     input! {
//         n: usize,
//         h: [usize; n],
//     }

//     let mut ans = vec![0; n];
//     let mut stack = vec![];
//     for i in (0..n - 1).rev() {
//         while let Some(&j) = stack.last() {
//             if h[j] > h[i + 1] {
//                 break;
//             }
//             stack.pop();
//         }
//         stack.push(i + 1);
//         ans[i] = stack.len();
//     }

//     println!("{}", ans.iter().join(" "));
// }

use ac_library::{Additive, LazySegtree, MapMonoid, Monoid};
use itertools::Itertools;
use proconio::input;
use std::collections::{BTreeSet, BinaryHeap};

/**
 * 区間加算
 *   - 区間和取得には対応していない => 区間幅が必要なので値を構造体で持つ
 * https://betrue12.hateblo.jp/entry/2020/09/23/005940
 */
struct F;
impl MapMonoid for F {
    type M = Additive<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        *f + *x
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f + *g
    }
}

/**
 * https://atcoder.github.io/ac-library/production/document_ja/lazysegtree.html
 * 遅延セグ木で殴る
 *
 * 1. 高いビルから処理していき、添字をBTreeSetに入れていく
 * 2. そうすると、「自分より左側に存在し、自分より高いビルの中で、添字が最大のビル」を取得できる
 * 3. あとは、そのビルの添字から自分自身-1までの区間に対して+1する
 * 4. この区間に対する+1は、セグ木で処理する
 */
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut max_heap = BinaryHeap::new();
    for (i, &h_i) in h.iter().enumerate() {
        max_heap.push((h_i, i));
    }

    let mut set = BTreeSet::new();
    let mut lazy_segtree = LazySegtree::<F>::new(n);
    while let Some((_, i)) = max_heap.pop() {
        set.insert(i);
        let l = *set.range(..i).next_back().unwrap_or(&0);
        let r = i;
        lazy_segtree.apply_range(l..r, 1);
    }

    println!("{}", (0..n).map(|i| lazy_segtree.get(i)).join(" "));
}
