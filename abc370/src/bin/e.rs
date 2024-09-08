use std::collections::HashMap;

use proconio::input;

const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc370/tasks/abc370_e
 * https://atcoder.jp/contests/abc370/editorial/10858
 * 累積和が単調増加でない時点で、尺取り法は使えない
 * => 「和が K になる部分列の数を列挙する」がそもそも難しい
 *
 * 他の方針として、DPが使えそうだが、どうもO(N^2)になってしまいそう
 * => こういう場合は、一度O(N^2)で書いてみることが大事
 * => その上で、DPの遷移を見直す（累積和を使ってO(N)にするなど）
 *
 * O(N^2)のDPを書いてみると、以下のようになる
 *   - dp[i]: 左から見て行って、最後に区切りを入れたのが地点 i である通り数
 */
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    // O(N^2)のDP
    // let mut dp = vec![0; n + 1];
    // dp[0] = 1;
    // for i in 1..=n {
    //     let mut sum = 0;
    //     for j in (0..i).rev() {
    //         sum += a[j];
    //         if sum != k {
    //             dp[i] += dp[j];
    //             dp[i] %= MOD;
    //         }
    //     }
    // }

    // O(N)のDP
    // a の累積和 s が使えそう
    // ただ累積和そのまま使うと、各 i に対して、j を全探索する必要がある
    // そこで、累積和の値をキーにして、その値が何回出現するかを記録するmapを使う
    // このmapを使うことで、除きたい場合の数 (s[i] - s[j] == k) を O(1) で求めることができる
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }
    let mut s_dp_map = HashMap::new();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    s_dp_map.insert(0, 1);
    let mut dp_sum = 1;
    for i in 1..=n {
        let s_excluded = s[i] - k;
        if let Some(&count) = s_dp_map.get(&s_excluded) {
            dp[i] = dp_sum + MOD - count;
            dp[i] %= MOD;
        } else {
            dp[i] = dp_sum;
            dp[i] %= MOD;
        }

        s_dp_map
            .entry(s[i])
            .and_modify(|s_dp| {
                *s_dp += dp[i];
                *s_dp %= MOD;
            })
            .or_insert(dp[i]);

        dp_sum += dp[i];
        dp_sum %= MOD;
    }

    println!("{}", dp[n]);
}
