use ac_library::{Additive, Segtree};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc392/tasks/abc392_f
/// https://atcoder.jp/contests/abc392/editorial/12145
///
/// 意外と惜しいところまで発想は至っていた
/// 1. 前から考えると操作ごとに一個前の操作の答えが変わる（O(N^2)になる）ので、後ろから考えてみる
/// 2. 後ろから決めていき、一度使った数字は取り除いて、詰める ということをやっていけば、O(N)で解けそう
/// 3. しかし、詰めるという作業を高速化する方法が思いつかなかった
/// => セグ木で使った数字を0にして、1から次に使う数字までの累積和（使ってない数字(1)の1からの数）を求めることで、詰めるという操作を高速化する
fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    }

    let mut segtree = Segtree::<Additive<usize>>::new(n);
    for i in 0..n {
        segtree.set(i, 1);
    }

    let mut ans = vec![n; n];
    for (i, &p_i) in p.iter().enumerate().rev() {
        // 使っていない数字のうち、前から数えて p_i 番目の数字を使う
        let j = segtree.max_right(0, |&x| x <= p_i);
        segtree.set(j, 0);
        ans[j] = i + 1;
    }

    println!("{}", ans.iter().join(" "));
}
