use ac_library::modint::ModInt998244353 as Mint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = Mint::new(0);
    for m in 1..=n {
        let mut dp = vec![vec![Mint::new(0); m]; m + 1];
        dp[0][0] = Mint::new(1);

        for i in 0..n {
            let mut next_dp = vec![vec![Mint::new(0); m]; m + 1];
            for j in 0..=m {
                for k in 0..m {
                    next_dp[j][k] += dp[j][k];

                    if j < m {
                        let next_k = (k + a[i]) % m;
                        next_dp[j + 1][next_k] += dp[j][k];
                    }
                }
            }
            dp = next_dp;
        }

        ans += dp[m][0];
    }

    println!("{}", ans.val());
}
