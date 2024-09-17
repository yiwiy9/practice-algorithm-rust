use proconio::input;

const MOD: usize = 998244353;

// abc307_e の類題らしい
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    }

    // k回目の移動で、(x==0 ? x2 : !x2 ) であり(y==0 ? y2 : !y2 ) であるものの個数
    let mut dp = vec![vec![vec![0; 2]; 2]; k + 1];
    if x1 == x2 && y1 == y2 {
        dp[0][1][1] = 1;
    } else if x1 == x2 {
        dp[0][1][0] = 1;
    } else if y1 == y2 {
        dp[0][0][1] = 1;
    } else {
        dp[0][0][0] = 1;
    }

    for i in 0..k {
        dp[i + 1][1][1] += dp[i][0][1] + dp[i][1][0];
        dp[i + 1][1][1] %= MOD;

        dp[i + 1][1][0] += dp[i][0][0] + dp[i][1][0] * (w - 2) + dp[i][1][1] * (w - 1);
        dp[i + 1][1][0] %= MOD;

        dp[i + 1][0][1] += dp[i][0][0] + dp[i][0][1] * (h - 2) + dp[i][1][1] * (h - 1);
        dp[i + 1][0][1] %= MOD;

        dp[i + 1][0][0] +=
            dp[i][0][0] * (w - 2 + h - 2) + dp[i][0][1] * (w - 1) + dp[i][1][0] * (h - 1);
        dp[i + 1][0][0] %= MOD;
    }

    println!("{}", dp[k][1][1]);
}
