use proconio::input;
const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc321/tasks/abc321_f
 * https://atcoder.jp/contests/abc321/editorial/7262
 *
 * dp[ 部分和 ] = { 場合の数 }
 *
 * x を追加 => i=K,K−1,…,x+1,x の順に、 dp[i] に dp[i−x] を加算
 * x を削除 => i=x,x+1,…,K−1,K の順に、dp[i] から dp[i−x] を減算 （追加とは逆順に削除）
 */
fn main() {
    input! {
        q: usize,
        k: usize,
        tx: [(char,usize); q],
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for (t, x) in tx {
        match t {
            '+' => {
                if x <= k {
                    for i in (x..=k).rev() {
                        dp[i] += dp[i - x];
                        dp[i] %= MOD;
                    }
                }
                println!("{}", dp[k]);
            }
            '-' => {
                if x <= k {
                    for i in x..=k {
                        dp[i] += MOD - dp[i - x];
                        dp[i] %= MOD;
                    }
                }
                println!("{}", dp[k]);
            }
            _ => unreachable!(),
        }
    }
}
