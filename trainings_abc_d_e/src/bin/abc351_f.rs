use ac_library::FenwickTree;
use proconio::input;
use std::collections::BTreeSet;
use superslice::*;

/**
 * https://atcoder.jp/contests/abc351/tasks/abc351_f
 * https://atcoder.jp/contests/abc351/editorial/9877
 * この問題は平面走査と呼ばれるアルゴリズムの基本問題です。
 *
 * 平面走査について
 * https://qiita.com/Shirotsume/items/9323f4268883d0d13500
 *
 * Fenwick Tree (BIT, フェニック木) 使い方
 * https://atcoder.github.io/ac-library/production/document_ja/fenwicktree.html
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let b = BTreeSet::from_iter(a.iter().cloned())
        .into_iter()
        .collect::<Vec<_>>();

    let m = b.len();

    let mut ans = 0;
    let mut ft_cnt = FenwickTree::new(m, 0);
    let mut ft_sum = FenwickTree::new(m, 0);
    for &a_i in a.iter().rev() {
        let compress_idx = b.lower_bound(&a_i);

        let cnt = ft_cnt.sum(compress_idx..m);
        let sum = ft_sum.sum(compress_idx..m);

        ans += sum - a_i * cnt;
        ft_cnt.add(compress_idx, 1);
        ft_sum.add(compress_idx, a_i);
    }

    println!("{}", ans);
}
