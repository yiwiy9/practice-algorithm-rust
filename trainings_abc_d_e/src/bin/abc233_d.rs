use proconio::input;
use std::collections::HashMap;

/**
 * https://atcoder.jp/contests/abc233/tasks/abc233_d
 * https://atcoder.jp/contests/abc233/editorial/3163
 * 累積和を使って、区間和がKになる個数を求める O(N)
 * => これだと間に合わないので、mapを使って高速化を行う
 */
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let mut map = HashMap::new();
    let mut ans = 0_usize;
    for s_i in &s {
        if let Some(&v) = map.get(&(s_i - k)) {
            ans += v;
        }
        map.entry(s_i).and_modify(|cur| *cur += 1).or_insert(1);
    }

    println!("{}", ans);
}
