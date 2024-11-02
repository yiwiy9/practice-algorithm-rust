use ac_library::{LazySegtree, MapMonoid, Monoid};
use itertools::Itertools;
use proconio::input;
use std::collections::{BTreeSet, BinaryHeap};

/**
 * 区間加算・区間和取得
 * 区間幅が必要なので値を構造体で持ちます
 *
 * チートシート
 *  - https://betrue12.hateblo.jp/entry/2020/09/23/005940
 * 使い方の詳細
 *  - https://betrue12.hateblo.jp/entry/2020/09/22/194541
 */
#[derive(Clone)]
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
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        S {
            value: x.value + f * x.size,
            size: x.size,
        }
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f + *g
    }
}

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut heap = BinaryHeap::new();
    for (i, &h_i) in h.iter().enumerate() {
        heap.push((h_i, i));
    }

    let mut lazy_segtree = LazySegtree::<F>::from(vec![S { value: 0, size: 1 }; n]);
    let mut set = BTreeSet::new();
    while let Some((_, i)) = heap.pop() {
        let left = set.range(..i).next_back().copied().unwrap_or(0);
        lazy_segtree.apply_range(left..i, 1);
        set.insert(i);
    }

    println!("{}", (0..n).map(|i| lazy_segtree.get(i).value).join(" "));
}
