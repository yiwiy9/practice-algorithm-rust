use proconio::input;
const MOD: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut dp = vec![vec![0; 3001]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        let mut s = 0;
        for j in 0..=3000 {
            s += dp[i - 1][j];
            s %= MOD;
            if a[i - 1] <= j && j <= b[i - 1] {
                dp[i][j] = s;
            }
        }
    }

    println!("{}", dp[n].iter().fold(0, |acc, &cur| (acc + cur) % MOD));
}
