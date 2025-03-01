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

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((h[i], i));
    }

    let mut lazy_segtree = LazySegtree::<F>::new(n);
    let mut set = BTreeSet::new();
    while let Some((_, i)) = heap.pop() {
        let l = *set.range(..i).next_back().unwrap_or(&0);
        let r = i;
        lazy_segtree.apply_range(l..r, 1);
        set.insert(i);
    }

    println!("{}", (0..n).map(|i| lazy_segtree.get(i)).join(" "));
}
