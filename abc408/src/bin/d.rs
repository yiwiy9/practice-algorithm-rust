use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;

/// 0を1にして決まるケースあるよ
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }

        let mut a = vec![];
        let mut cur = 1;
        for i in 1..n {
            if s[i] == s[i - 1] {
                cur += 1;
            } else {
                a.push((s[i - 1], cur));
                cur = 1;
            }
        }
        a.push((s[n - 1], cur));

        // dp[i][j][k] := i番目まで見て、j番目の文字が0か1か、1→0に変えたかどうか：k
        // 010 が 1ではなく0なので、変えたかどうかではなく、1→0に変えたかを持つ（敗北ポイント）
        let mut dp = vec![vec![vec![INF; 2]; 2]; a.len()];
        if a[0].0 == '1' {
            dp[0][0][0] = a[0].1;
            dp[0][1][0] = 0;
        } else {
            dp[0][0][0] = 0;
            dp[0][1][0] = a[0].1;
        }

        for i in 1..a.len() {
            if a[i].0 == '1' {
                dp[i][0][0] = dp[i - 1][0][0] + a[i].1;
                dp[i][0][1] = dp[i - 1][0][1].min(dp[i - 1][1][0]) + a[i].1;

                dp[i][1][0] = dp[i - 1][1][0].min(dp[i - 1][0][0]);
                dp[i][1][1] = dp[i - 1][1][1];
            } else {
                dp[i][0][0] = dp[i - 1][0][0];
                dp[i][0][1] = dp[i - 1][0][1].min(dp[i - 1][1][0]);

                dp[i][1][0] = dp[i - 1][1][0].min(dp[i - 1][0][0]) + a[i].1;
                dp[i][1][1] = dp[i - 1][1][1] + a[i].1;
            }
        }

        println!(
            "{}",
            dp[a.len() - 1][0][0]
                .min(dp[a.len() - 1][1][0])
                .min(dp[a.len() - 1][0][1])
                .min(dp[a.len() - 1][1][1])
        );
    }
}

// /**　落ちていたケース
// 1
// 7
// 0110110
//  */　0に1が挟まれるケース (0111110)
