use itertools::Itertools as _;
use proconio::input;
use superslice::*;

const INF: usize = 1 << 60;

/**
 * https://atcoder.jp/contests/abc369/tasks/abc369_f
 * https://atcoder.jp/contests/abc369/editorial/10835
 * 本問題は数列 c の最長 (広義) 増加部分列 (LIS) の一例を求める問題に帰着されます
 *
 * 与えられた正整数列 c=(c1,c2,…,cN) に対し、c の LIS の長さ（LIS 自体ではない）を求めるアルゴリズムとして以下のような有名なアルゴリズムがあります。
 *   1.長さ N の数列 d を用意し、その全ての要素を ∞ で初期化する。
 *   2. i=1,2,…,N の順に以下を行う。
 *       - dj > ci を満たす最小の j に対して、dj を ci で更新する。
 *   3. c の LIS の長さは、dj < ∞ を満たす最大の j の値である。
 * => 意味合いとしては、このアルゴリズムにおける各 dj は、c1,c2,…,ci の中で作れる
 *    長さ j の (広義) 増加部分列の末尾の要素の最小値 (そのような部分列が存在しないならば ∞) を管理しています。
 *
 * => 今回は、c の LIS の長さを求めるのに加えて、c の LIS の一例を求める必要がある
 */
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut rc: [(usize, usize); n],
    }

    rc.sort();

    let mut dp = vec![INF; n];
    let mut idx = vec![n; n];
    let mut prev = vec![n; n];
    for i in 0..n {
        let c_upper_idx = dp.upper_bound(&rc[i].1);
        dp[c_upper_idx] = rc[i].1;
        idx[c_upper_idx] = i;
        if c_upper_idx > 0 {
            prev[i] = idx[c_upper_idx - 1];
        }
    }

    let m = idx.iter().position(|&x| x == n).unwrap_or(n) - 1;
    let mut path = vec![];
    path.push((h, w));
    let mut cur = idx[m];
    while cur != n {
        path.push(rc[cur]);
        cur = prev[cur];
    }
    path.push((1, 1));

    path.reverse();
    let mut ans = vec![];
    for i in 1..path.len() {
        let mut down_diff = path[i].0 - path[i - 1].0;
        let mut right_diff = path[i].1 - path[i - 1].1;
        while down_diff > 0 {
            ans.push('D');
            down_diff -= 1;
        }
        while right_diff > 0 {
            ans.push('R');
            right_diff -= 1;
        }
    }

    println!("{}", m + 1);
    println!("{}", ans.iter().join(""));
}
