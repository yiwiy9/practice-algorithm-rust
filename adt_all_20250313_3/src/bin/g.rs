use proconio::{input, marker::Chars};
use std::collections::HashMap;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let modified_dp = move |mut dp: Vec<char>, c: char| {
        dp.push(c);

        match dp.len().cmp(&k) {
            std::cmp::Ordering::Less => return (dp, true),
            std::cmp::Ordering::Greater => {
                dp.remove(0);
            }
            std::cmp::Ordering::Equal => {}
        }

        let mut is_palindrome = true;

        for i in 0..dp.len() / 2 {
            if dp[i] != dp[dp.len() - 1 - i] {
                is_palindrome = false;
                break;
            }
        }

        (dp, !is_palindrome)
    };

    let mut dp_map = HashMap::new();
    dp_map.insert(vec![], 1);
    for &c in &s {
        let mut next_dp_map = HashMap::new();
        for (dp, count) in dp_map {
            match c {
                '?' => {
                    let (next_dp, ok) = modified_dp(dp.clone(), 'A');
                    if ok {
                        next_dp_map
                            .entry(next_dp)
                            .and_modify(|e| {
                                *e += count;
                                *e %= MOD;
                            })
                            .or_insert(count);
                    }
                    let (next_dp, ok) = modified_dp(dp.clone(), 'B');
                    if ok {
                        next_dp_map
                            .entry(next_dp)
                            .and_modify(|e| {
                                *e += count;
                                *e %= MOD;
                            })
                            .or_insert(count);
                    }
                }
                _ => {
                    let (next_dp, ok) = modified_dp(dp.clone(), c);
                    if ok {
                        next_dp_map
                            .entry(next_dp)
                            .and_modify(|e| {
                                *e += count;
                                *e %= MOD;
                            })
                            .or_insert(count);
                    }
                }
            }
        }
        dp_map = next_dp_map;
    }

    println!(
        "{}",
        dp_map
            .iter()
            .fold(0, |acc, (_, &count)| (acc + count) % MOD)
    );
}
