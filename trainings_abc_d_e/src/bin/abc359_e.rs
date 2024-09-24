// use itertools::Itertools;
// use proconio::input;

// /**
//  * https://atcoder.jp/contests/abc359/tasks/abc359_e
//  * https://atcoder.jp/contests/abc359/editorial/10262
//  * ABC372_D の類題
//  */
// fn main() {
//     input! {
//         n: usize,
//         h: [usize; n],
//     }

//     let mut ans = vec![0; n + 1];
//     ans[0] = 1;

//     let mut stack = vec![];

//     for i in 1..=n {
//         ans[i] = ans[i - 1];

//         let mut cnt = 1;
//         while let Some(&(h_j, cnt_j)) = stack.last() {
//             if h_j > h[i - 1] {
//                 break;
//             }
//             cnt += cnt_j;
//             ans[i] -= h_j * cnt_j;
//             stack.pop();
//         }

//         ans[i] += h[i - 1] * cnt;
//         stack.push((h[i - 1], cnt));
//     }

//     println!("{}", ans.iter().skip(1).join(" "));
// }

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

/**
 * https://atcoder.github.io/ac-library/production/document_ja/lazysegtree.html
 * 遅延セグ木で殴る
 *
 * 1. 区間最大値を求めるセグ木を使い、高さを値として持つ
 * 2. 板を左から処理していき、セグ木から「自分から左側で自分より低い板が連続する区間の中で、添字が最小の板」を取得できる
 * 3. あとは、その板の添字+1から自分自身までの区間に対して高さを自分自身と同じにする
 * 4. この区間に対する高さの更新は、セグ木で処理する
 * 5. その後、先頭から自分自身までの区間和を取得し、+1したものを出力する
 */
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    // 区間最大値を求めるセグ木。min_leftで比較したいのは「自分より小さい値」なので、Maxを使う
    let max_segtree = Segtree::<Max<usize>>::from(h.clone());

    let mut a = vec![0; n];
    let mut lazy_segtree = LazySegtree::<F>::from(vec![S { value: 0, size: 1 }; n]);
    for (i, &h_i) in h.iter().enumerate() {
        let l = max_segtree.min_left(i, |&h_j| h_j < h_i);
        lazy_segtree.apply_range(l..=i, Some(h_i));
        a[i] = lazy_segtree.prod(0..=i).value + 1;
    }

    println!("{}", a.iter().join(" "));
}
