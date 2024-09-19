use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }

    // dp[i][k]: i回目までのコンテストでk個選んだ場合のレートの最大値
    let mut dp = vec![vec![0.0_f64; n + 1]; n + 1];
    for i in (1..=n).rev() {
        let mut c = 1.0;
        for k in 0..=(n - i) {
            dp[i - 1][k] = dp[i - 1][k].max(dp[i][k]);
            dp[i - 1][k + 1] = dp[i - 1][k + 1].max(dp[i][k] + p[i - 1] * c);
            c *= 0.9;
        }
    }

    let mut c = 1.0;
    let mut div = c;
    let mut ans = -1210.0_f64;
    for k in 1..=n {
        ans = ans.max((dp[0][k] / div) - (1200.0 / (k as f64).sqrt()));
        c *= 0.9;
        div += c;
    }

    println!("{:.10}", ans);
}
