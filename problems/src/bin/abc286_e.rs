use proconio::{
    input,
    marker::{Chars, Usize1},
};

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: [Chars; n],
        q: usize,
        uv: [(Usize1, Usize1); q],
    }

    let mut dp = vec![vec![(INF, 0); n]; n];
    for (i, &a_i) in a.iter().enumerate() {
        dp[i][i] = (0, 0);
    }
    for (i, s_i) in s.iter().enumerate() {
        for (j, &c) in s_i.iter().enumerate() {
            if c == 'Y' {
                dp[i][j] = (1, a[j]);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let next_dist = dp[i][k].0 + dp[k][j].0;
                let next_value = dp[i][k].1 + dp[k][j].1;
                if dp[i][j].0 > next_dist || (dp[i][j].0 == next_dist && dp[i][j].1 < next_value) {
                    dp[i][j] = (next_dist, next_value);
                }
            }
        }
    }

    for &(u, v) in &uv {
        if dp[u][v].0 == INF {
            println!("Impossible");
        } else {
            println!("{} {}", dp[u][v].0, dp[u][v].1 + a[u]);
        }
    }
}
