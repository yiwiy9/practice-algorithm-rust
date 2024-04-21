use proconio::input;
use std::collections::BTreeMap;

/**
 * https://atcoder.jp/contests/abc350/tasks/abc350_e
 * https://atcoder.jp/contests/abc350/editorial/9812
 * 期待値DPをメモ化再帰に置き換えて解く（nがでかいので配列で管理できない）
 *
 * 求める期待値をf(n)とすると、
 * f(n) = Y + f(n/1)/6 + f(n/2)/6 + f(n/3)/6 + f(n/4)/6 + f(n/5)/6 + f(n/6)/6
 * 右辺にf(n)が含まれている（自己ループを持つ）ため、式変形を行う
 * f(n) = Y*6/5 + f(n/2)/5 + f(n/3)/5 + f(n/4)/5 + f(n/5)/5 + f(n/6)/5
 *
 * もう一方の操作を加えて、期待値の小さい方を採用するようにすると
 * f(n) = min{ X + f(n/a), Y*5/6 + f(n/2)/5 + f(n/3)/5 + f(n/4)/5 + f(n/5)/5 + f(n/6)/5 }
 */
fn main() {
    input! {
        n: usize,
        a: usize,
        x: f64,
        y: f64,
    }

    let mut memo = BTreeMap::new();
    println!("{:.10}", rec(a, x, y, &mut memo, n));
}

fn rec(a: usize, x: f64, y: f64, memo: &mut BTreeMap<usize, f64>, n: usize) -> f64 {
    if n == 0 {
        return 0.0;
    }

    if let Some(&v) = memo.get(&n) {
        return v;
    }

    let first = x + rec(a, x, y, memo, n / a);

    let second = y * 6.0 / 5.0
        + rec(a, x, y, memo, n / 2) / 5.0
        + rec(a, x, y, memo, n / 3) / 5.0
        + rec(a, x, y, memo, n / 4) / 5.0
        + rec(a, x, y, memo, n / 5) / 5.0
        + rec(a, x, y, memo, n / 6) / 5.0;

    let res = first.min(second);
    memo.insert(n, res);
    res
}
