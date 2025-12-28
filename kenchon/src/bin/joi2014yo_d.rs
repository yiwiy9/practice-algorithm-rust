use std::collections::HashMap;

use proconio::{input, marker::Chars};

const MOD: usize = 10007;

/// https://atcoder.jp/contests/joi2014yo/tasks/joi2014yo_d
/// https://drken1215.hatenablog.com/entry/2020/12/03/182300
/// => 遷移パターンが多すぎて複雑な場合は bit DP を疑う
///
/// Tips: 幅が短くて、長さがめっちゃ長い」という長方形形状の色塗り問題は、bit DP をまずは疑う
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let map = HashMap::from([('J', 0), ('O', 1), ('I', 2)]);
    let mut dp = vec![0; 1 << 3];
    dp[1] = 1;

    for c in &s {
        let in_charge_i = *map.get(c).unwrap();

        let mut next_dp = vec![0; 1 << 3];
        for from_bit in 0..(1 << 3) {
            for to_bit in 0..(1 << 3) {
                if to_bit & (1 << in_charge_i) == 0 {
                    // 責任者は必ず出席する必要がある
                    continue;
                }

                if to_bit & from_bit == 0 {
                    // 鍵を持っている人は連日の参加が必要
                    continue;
                }

                next_dp[to_bit] += dp[from_bit];
                next_dp[to_bit] %= MOD;
            }
        }
        dp = next_dp;
    }

    println!("{}", dp.iter().sum::<usize>() % MOD);
}
