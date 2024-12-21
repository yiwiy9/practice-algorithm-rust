use proconio::input;

const MOD: usize = 998244353;

/// https://atcoder.jp/contests/adt_all_20241121_1/tasks/abc262_d
/// https://atcoder.jp/contests/adt_all_20241121_1/editorial/4503
///
/// N<=100, time <= 2.5 sec
/// => O(N^4) いけるということを考える
///
/// 割る数が変わってしまうと余りが同じでも答えが変わってしまう
/// => DPの遷移中に割る数を変えることはできない
///
/// 割る数でDPを分ける
/// => i=1,2,...,N個選んだ場合の余り0を足し上げる
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for i in 1..=n {
        // dp[k][l] := k個選んだ時の余りlの個数
        let mut dp = vec![vec![0; i]; i + 1];
        dp[0][0] = 1;
        for &a_i in a.iter() {
            let mut next_dp = vec![vec![0; i]; i + 1];
            for k in 0..=i {
                for l in 0..i {
                    next_dp[k][l] += dp[k][l];
                    next_dp[k][l] %= MOD;

                    if k + 1 <= i {
                        next_dp[k + 1][(l + a_i) % i] += dp[k][l];
                        next_dp[k + 1][(l + a_i) % i] %= MOD;
                    }
                }
            }
            dp = next_dp;
        }
        ans += dp[i][0];
        ans %= MOD;
    }

    println!("{}", ans);
}
