use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        txa: [(usize,usize,i64); n],
    }

    let mut tx_map = BTreeMap::new();
    for (t, x, a) in txa {
        tx_map.insert((t, x), a);
    }

    let mut dp = vec![vec![-1; 5]; 100_001];
    dp[0][0] = 0;
    for i in 0..100_000 {
        for j in 0..5 {
            if dp[i][j] == -1 {
                continue;
            }
            if j != 0 {
                dp[i + 1][j - 1] =
                    dp[i + 1][j - 1].max(dp[i][j] + tx_map.get(&(i + 1, j - 1)).unwrap_or(&0));
            }
            if j != 4 {
                dp[i + 1][j + 1] =
                    dp[i + 1][j + 1].max(dp[i][j] + tx_map.get(&(i + 1, j + 1)).unwrap_or(&0));
            }
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + tx_map.get(&(i + 1, j)).unwrap_or(&0));
        }
    }

    println!("{:?}", dp[100_000].iter().max().unwrap());
}
