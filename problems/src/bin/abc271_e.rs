use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        abc: [(Usize1, Usize1, usize); m],
        e: [Usize1; k],
    }

    let mut dp = vec![INF; n];
    dp[0] = 0;
    for j in 0..k {
        let (a, b, c) = abc[e[j]];
        if dp[a] != INF {
            dp[b] = dp[b].min(dp[a] + c);
        }
    }

    println!(
        "{}",
        if dp[n - 1] == INF {
            -1
        } else {
            dp[n - 1] as i64
        }
    );
}
