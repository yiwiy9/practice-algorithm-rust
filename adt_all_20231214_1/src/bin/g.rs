use proconio::input;
const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/adt_all_20231214_1/tasks/abc262_d
 * https://atcoder.jp/contests/adt_all_20231214_1/editorial/4503
 * 余りのの数に着目したDP
 *
 * d = 1..nにおいて
 * dp[i][j][k]: i番目までの数をj個選んで、その合計をdで割った余りがkになるような選び方の数
 *
 * nの余りで計算してはいけない
 * => 選ぶ項数がそれぞれ異なるため
 * => d=1..nにおいて、dp[n][d][0]の総和が答え
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    for d in 1..=n {
        let mut dp = vec![vec![vec![0; d]; d + 1]; n + 1];
        dp[0][0][0] = 1;
        for i in 0..n {
            for j in 0..=d {
                for k in 0..d {
                    dp[i + 1][j][k] += dp[i][j][k];
                    dp[i + 1][j][k] %= MOD;

                    if j != d {
                        dp[i + 1][j + 1][(k + a[i]) % d] += dp[i][j][k];
                        dp[i + 1][j + 1][(k + a[i]) % d] %= MOD;
                    }
                }
            }
        }
        ans += dp[n][d][0];
        ans %= MOD;
    }

    println!("{}", ans);
}
