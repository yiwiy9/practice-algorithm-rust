use std::collections::HashMap;

use proconio::{input, marker::Chars};

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut dp = HashMap::new();
    dp.insert(vec![], 1);

    for &c in &s {
        let mut next_dp = HashMap::new();

        for (key, &val) in &dp {
            let mut new_key = key.clone();

            if c == '?' {
                new_key.push('A');
                update(k, &mut next_dp, &mut new_key, val);
                new_key = key.clone();
                new_key.push('B');
                update(k, &mut next_dp, &mut new_key, val);
            } else {
                new_key.push(c);
                update(k, &mut next_dp, &mut new_key, val);
            }
        }

        dp = next_dp;
    }

    println!("{}", dp.values().fold(0, |acc, &v| (acc + v) % MOD));
}

fn update(k: usize, next_dp: &mut HashMap<Vec<char>, usize>, new_key: &mut Vec<char>, val: usize) {
    if new_key.len() > k {
        new_key.remove(0);
    }
    if new_key.len() == k && is_palindrome(&new_key) {
        return;
    }
    next_dp
        .entry(new_key.clone())
        .and_modify(|v| {
            *v += val;
            *v %= MOD;
        })
        .or_insert(val);
}

fn is_palindrome(s: &Vec<char>) -> bool {
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            return false;
        }
    }
    true
}
