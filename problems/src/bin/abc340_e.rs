use ac_library::{Additive, LazySegtree, MapMonoid, Monoid};
use itertools::Itertools;
use proconio::input;

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
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut lazy_segtree = LazySegtree::<F>::new(n);
    for (i, &a_i) in a.iter().enumerate() {
        lazy_segtree.set(i, a_i);
    }

    for &b_i in &b {
        let num = lazy_segtree.get(b_i);
        lazy_segtree.set(b_i, 0);

        if b_i + num < n {
            lazy_segtree.apply_range(b_i + 1..=b_i + num, 1)
        } else {
            lazy_segtree.apply_range(b_i + 1..n, 1);

            let rest = num - (n - b_i - 1);
            lazy_segtree.apply_range(0..n, rest / n);
            lazy_segtree.apply_range(0..rest % n, 1);
        };
    }

    println!("{}", (0..n).map(|i| lazy_segtree.get(i)).join(" "));
}
