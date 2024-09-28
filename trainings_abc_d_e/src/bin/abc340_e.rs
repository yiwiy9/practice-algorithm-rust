use ac_library::{LazySegtree, MapMonoid, Monoid};
use itertools::Itertools;
use proconio::input;

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
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut lazy_segtree = LazySegtree::<F>::from(
        a.iter()
            .map(|&a_i| S {
                value: a_i,
                size: 1,
            })
            .collect_vec(),
    );

    for &b_i in &b {
        let a_b_i = lazy_segtree.get(b_i).value;
        let all_apply = a_b_i / n;
        let rest = a_b_i % n;
        lazy_segtree.set(b_i, S { value: 0, size: 1 });

        lazy_segtree.apply_range(0..n, all_apply);
        if b_i + rest < n {
            lazy_segtree.apply_range(b_i + 1..=b_i + rest, 1);
        } else {
            lazy_segtree.apply_range(b_i + 1..n, 1);
            lazy_segtree.apply_range(0..=rest - (n - b_i), 1);
        }
    }

    println!("{}", (0..n).map(|i| lazy_segtree.get(i).value).join(" "));
}
