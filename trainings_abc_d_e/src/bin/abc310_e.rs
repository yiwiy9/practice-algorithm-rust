use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![vec![0; 2]; n + 1];
    for i in 0..n {
        match s[i] {
            '0' => {
                dp[i + 1][0] = 1;
                dp[i + 1][1] = dp[i][0] + dp[i][1];
            }
            '1' => {
                dp[i + 1][0] = dp[i][1];
                dp[i + 1][1] = 1 + dp[i][0];
            }
            _ => unreachable!(),
        }
    }

    println!("{}", dp.iter().map(|v| v[1]).sum::<usize>());
}
