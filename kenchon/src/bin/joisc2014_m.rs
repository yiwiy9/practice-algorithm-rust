use proconio::input;
use std::collections::BinaryHeap;

const INF: i64 = 1 << 60;

/// https://atcoder.jp/contests/joisc2014/tasks/joisc2014_m
/// https://www2.ioi-jp.org/camp/2014/2014-sp-tasks/2014-sp-d4.pdf
/// - 固定するもの:
///   取り付け順そのものではなく、現在の空き端子数だけを見る。
///   1 本取り付けたときの空き端子数の増減は `a_i - 1`。
/// - 欲しい量:
///   負のストラップで追加枠を j 個ぶん確保するときの嬉しさ最大値。
///   その j を使って `a_i = 0, b_i >= 0` を上から j 本採用したときの最大値を知りたい。
/// - 必要クエリ:
///   `b_i < 0, a_i > 1` だけで
///   `dp[j] = 追加枠を j 個確保するときの嬉しさ最大値`
///   を求め、`a_i = 0, b_i >= 0` の上位 j 本和と合成する。
/// - 単調性 / 境界:
///   `b_i >= 0, a_i > 0` は常に取ってよく、`b_i < 0, a_i <= 1` は取る意味がない。
///   また必要な追加枠は `a_i = 0, b_i >= 0` の本数まで見れば十分。
///
/// AC: 39分
fn main() {
    input! {
        n: usize,
        ab: [(usize,i64); n],
    }

    let mut ans = 0;
    let mut can_add = 1;
    let mut b_puls_a_zero = BinaryHeap::new();
    let mut b_minus = vec![];
    for &(a, b) in &ab {
        if b >= 0 {
            if a > 0 {
                ans += b;
                can_add += a - 1;
            } else {
                b_puls_a_zero.push(b);
            }
        } else if a > 1 {
            b_minus.push((a, b));
        }
    }

    while can_add > 0 {
        if let Some(b_i) = b_puls_a_zero.pop() {
            ans += b_i;
            can_add -= 1;
        } else {
            break;
        }
    }

    if b_puls_a_zero.is_empty() {
        println!("{}", ans);
        return;
    }

    let k = b_puls_a_zero.len();
    let mut dp = vec![-INF; k + 1];
    dp[0] = 0;
    for (a_i, b_i) in b_minus {
        let mut next_dp = dp.clone();
        for j in 0..=k {
            let next_j = (j + a_i - 1).min(k);
            next_dp[next_j] = next_dp[next_j].max(dp[j] + b_i);
        }
        dp = next_dp;
    }

    let mut cur_sum = ans;
    for j in 1..=k {
        cur_sum += b_puls_a_zero.pop().unwrap();
        ans = ans.max(cur_sum + dp[j]);
    }

    println!("{}", ans);
}
