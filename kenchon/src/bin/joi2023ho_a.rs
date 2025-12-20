use ac_library::{LazySegtree, MapMonoid, Monoid};
use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

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
        a: [usize; n],
    }

    let mut lazy_segtree = LazySegtree::<F>::from(vec![S { value: 0, size: 1 }; n]);
    let mut color_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (i, &a_i) in a.iter().enumerate() {
        lazy_segtree.set(
            i,
            S {
                value: a_i,
                size: 1,
            },
        );

        if let Some(stack) = color_map.get_mut(&a_i) {
            while let Some(last_i) = stack.pop() {
                if lazy_segtree.get(last_i).value == a_i {
                    lazy_segtree.apply_range(last_i + 1..i, Some(a_i));
                    break;
                }
            }
        }

        color_map.entry(a_i).or_default().push(i);
    }

    println!("{}", (0..n).map(|i| lazy_segtree.get(i).value).join("\n"));
}
