use itertools::Itertools;
use proconio::input;

const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc362/tasks/abc362_e
 * https://atcoder.jp/contests/abc362/editorial/10399
 *
 * dp[i][j][l]:= 初項Ai，第2項Aj，長さl(l≥2)であるような等差数列の個数
 *
 * => iの降順に見ていき，初項Aiを固定します．第2項Aj(i<j)と長さlを全探索します．
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n];

    for i in (0..n).rev() {
        dp[i][n][1] = 1;
        for j in (i + 1)..n {
            dp[i][j][2] = 1;
            for l in 3..=n {
                for k in (j + 1)..n {
                    // a[j] - a[i] == a[k] - a[j]
                    if a[j] + a[j] == a[i] + a[k] {
                        dp[i][j][l] += dp[j][k][l - 1];
                        dp[i][j][l] %= MOD;
                    }
                }
            }
        }
    }

    let mut ans = vec![0; n + 1];
    for i in 0..n {
        for j in (i + 1)..=n {
            for l in 1..=n {
                ans[l] += dp[i][j][l];
                ans[l] %= MOD;
            }
        }
    }

    println!("{}", ans.iter().skip(1).join(" "));
}
