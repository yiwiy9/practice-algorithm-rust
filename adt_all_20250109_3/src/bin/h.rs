use proconio::{input, marker::Usize1};
const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        abc: [(Usize1,Usize1,i64); m],
        e: [Usize1; k],
    }

    let mut dp = vec![INF; n];
    dp[n - 1] = 0;
    for &e_i in e.iter().rev() {
        let (a, b, c) = abc[e_i];
        if dp[b] != INF {
            dp[a] = dp[a].min(dp[b] + c);
        }
    }

    println!("{}", if dp[0] == INF { -1 } else { dp[0] });
}
