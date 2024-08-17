use proconio::input;
const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut dp = vec![vec![0; 3001]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        let mut before_sum = 0;
        for j in 0..=3000 {
            before_sum += dp[i][j];
            before_sum %= MOD;

            if a[i] <= j && j <= b[i] {
                dp[i + 1][j] = before_sum;
            }
        }
    }

    println!("{}", dp[n].iter().sum::<usize>() % MOD);
}
