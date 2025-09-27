use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        m: i64,
        ab: [(usize, i64); n],
    }

    let mut ans = 0;
    let mut dp = vec![vec![-1; h + 1]; n + 1];
    dp[0][h] = m;
    for (i, &(a, b)) in ab.iter().enumerate() {
        let mut ok = false;
        for j in 0..=h {
            if dp[i][j] < 0 {
                continue;
            }
            if j >= a {
                dp[i + 1][j - a] = dp[i + 1][j - a].max(dp[i][j]);
                ok = true;
            }
            if dp[i][j] >= b {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] - b);
                ok = true;
            }
        }
        if ok {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
