use proconio::input;
use std::collections::BTreeMap;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    let mut sum = 0;
    let mut all_cnt = 1;
    let mut sum_cnt_map = BTreeMap::new();
    sum_cnt_map.insert(0, 1);
    for i in 0..n {
        sum += a[i];
        let mut cur = all_cnt;
        if let Some(&cnt) = sum_cnt_map.get(&(sum - k)) {
            cur += MOD - cnt;
            cur %= MOD;
        }
        dp[i + 1] = cur;
        all_cnt += cur;
        all_cnt %= MOD;

        sum_cnt_map
            .entry(sum)
            .and_modify(|e| {
                *e += cur;
                *e %= MOD;
            })
            .or_insert(cur);
    }

    println!("{}", dp[n]);
}
