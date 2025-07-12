use ac_library::{LazySegtree, MapMonoid, Max, Monoid, Segtree};
use itertools::Itertools;
use proconio::input;

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

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = vec![0; n];
    let mut lazy_segtree = LazySegtree::<F>::from(vec![S { value: 0, size: 1 }; n]);
    let max_segtree = Segtree::<Max<usize>>::from(h.clone());
    for i in 0..n {
        let left_i = max_segtree.min_left(i, |x| x < &h[i]);
        lazy_segtree.apply_range(left_i..i + 1, Some(h[i]));
        ans[i] = lazy_segtree.prod(0..i + 1).value + 1;
    }

    println!("{}", ans.iter().join(" "));
}
