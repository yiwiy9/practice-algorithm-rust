use proconio::input;
const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc262/tasks/abc262_d
 * https://atcoder.jp/contests/abc262/editorial/4503
 *
 * 3重ループのままでやろうとすると、j->j+1 の遷移で割る数が変わるので、持ってる情報が余りだけだと足りない
 * そういうときは、大外にループを追加して、割る数を1~Nで固定して遷移を行うようにする
 *
 * 余りを遷移させるときは商の情報が喪失するため、必ず割る数は固定する必要がある
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for cur_mod in 1..=n {
        // cur_mod ごとに初期化
        let mut dp = vec![vec![vec![0; n]; n + 1]; n + 1];
        dp[0][0][0] = 1;
        for i in 0..n {
            for j in 0..n + 1 {
                for k in 0..n {
                    // ここを、cur_mod で割った余りとすることで計算ができる
                    // // 余りを遷移させるときは商の情報が喪失するため、必ず割る数は固定する
                    dp[i + 1][j][k] += dp[i][j][k];
                    dp[i + 1][j][k] %= MOD;

                    if j < n {
                        dp[i + 1][j + 1][(k + a[i]) % cur_mod] += dp[i][j][k];
                        dp[i + 1][j + 1][(k + a[i]) % cur_mod] %= MOD;
                    }
                }
            }
        }

        ans += dp[n][cur_mod][0];
        ans %= MOD;
    }

    println!("{}", ans);
}
