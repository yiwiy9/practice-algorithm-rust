use proconio::input;

/**
 * https://atcoder.jp/contests/adt_medium_20240201_2/tasks/abc334_c
 * https://atcoder.jp/contests/adt_medium_20240201_2/editorial/8983
 * 先頭からある要素までの累積和と、末尾からある要素までの累積和を求めておけば
 * 奇数の時に省きたい要素を全探索しても計算量は O(N) で済む
 *
 * 偶数の時は、当たり前にpreとsufは同じ
 * 奇数の時は、preは0-1,2-4,...,(n-3)-(n-2)でsufは1-2,3-4,...,(n-2)-(n-1)とカバーする範囲が異なる
 */
fn main() {
    input! {
        _: usize,
        k: usize,
        a: [usize; k],
    }

    let mut pre = vec![0; k + 1];
    for i in 1..=k {
        pre[i] = pre[i - 1];
        if i % 2 == 0 {
            pre[i] += a[i - 1] - a[i - 2];
        }
    }
    let mut suf = vec![0; k + 1];
    for i in (0..k).rev() {
        suf[i] = suf[i + 1];
        if (k - i) % 2 == 0 {
            suf[i] += a[i + 1] - a[i];
        }
    }

    if k % 2 == 0 {
        println!("{}", pre[k]);
        return;
    }

    let mut ans = 1 << 30;
    for i in 0..=k {
        ans = ans.min(pre[i] + suf[i]);
    }

    println!("{}", ans);
}
