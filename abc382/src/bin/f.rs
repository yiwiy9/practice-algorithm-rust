use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

/**
 * 区間変更・区間最大値取得
 *
 * チートシート
 *  - https://betrue12.hateblo.jp/entry/2020/09/23/005940
 * 使い方の詳細
 *  - https://betrue12.hateblo.jp/entry/2020/09/22/194541
 */
#[derive(Clone)]
struct F;
impl MapMonoid for F {
    type M = Max<usize>;
    type F = Option<usize>;

    fn identity_map() -> Self::F {
        // lazyの値として取り得ないような値を擬似的に恒等写像として扱う
        None
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        match f {
            Some(v) => *v,
            None => *x,
        }
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f.or(*g)
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rcl: [(Usize1, Usize1, usize); n],
    }

    let mut rcli = rcl
        .iter()
        .enumerate()
        .map(|(i, &(r, c, l))| (r, c, l, i))
        .collect::<Vec<_>>();
    rcli.sort_by(|a, b| b.0.cmp(&a.0));

    let mut lazy_segtree = LazySegtree::<F>::new(w);
    let mut ans = vec![0; n];
    for &(_, c, l, i) in &rcli {
        let new_height = lazy_segtree.prod(c..c + l);
        ans[i] = h - new_height;
        lazy_segtree.apply_range(c..c + l, Some(new_height + 1));
    }

    println!("{}", ans.iter().join("\n"));
}
