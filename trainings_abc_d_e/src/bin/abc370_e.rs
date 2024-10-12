use std::collections::BTreeMap;

use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut dp = vec![0; n + 1];
    let mut map = BTreeMap::new();

    dp[0] = 1;
    map.insert(0, 1);
    let mut all = 1;
    let mut acc = 0;
    for i in 0..n {
        acc += a[i];
        if let Some(dp_j) = map.get(&(acc - k)) {
            dp[i + 1] = (all + MOD - dp_j) % MOD;
        } else {
            dp[i + 1] = all;
        }

        map.entry(acc)
            .and_modify(|cur| *cur = (*cur + dp[i + 1]) % MOD)
            .or_insert(dp[i + 1]);

        all += dp[i + 1];
        all %= MOD;
    }

    println!("{}", dp[n]);
}
