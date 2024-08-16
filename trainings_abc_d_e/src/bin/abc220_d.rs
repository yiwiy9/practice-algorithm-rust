use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![0; 10]; n];
    dp[0][a[0]] = 1;

    for i in 1..n {
        for j in 0..10 {
            if dp[i - 1][j] == 0 {
                continue;
            }

            let f = (j + a[i]) % 10;
            dp[i][f] += dp[i - 1][j];
            dp[i][f] %= MOD;

            let g = (j * a[i]) % 10;
            dp[i][g] += dp[i - 1][j];
            dp[i][g] %= MOD;
        }
    }

    for i in 0..10 {
        println!("{}", dp[n - 1][i]);
    }
}
