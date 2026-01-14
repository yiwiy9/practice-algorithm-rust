use proconio::{input, marker::Chars};

const MOD: usize = 2019;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans: usize = 0;
    let mut dp = vec![0; MOD];
    for &c in &s {
        let num = c as usize - '0' as usize;
        let mut next_dp = vec![0; MOD];
        next_dp[num] = 1;
        for (j, &cnt) in dp.iter().enumerate() {
            next_dp[(j * 10 + num) % MOD] += cnt;
        }
        dp = next_dp;
        ans += dp[0];
    }

    println!("{}", ans);
}
