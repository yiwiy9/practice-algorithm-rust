use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;
const MATCH_S: [char; 7] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![vec![0; MATCH_S.len() + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for (j, _) in MATCH_S.iter().enumerate() {
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= MOD;
            if s[i] == MATCH_S[j] {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j + 1] %= MOD;
            }
        }
        dp[i + 1][MATCH_S.len()] += dp[i][MATCH_S.len()];
        dp[i + 1][MATCH_S.len()] %= MOD;
    }

    println!("{}", dp[n][MATCH_S.len()])
}
