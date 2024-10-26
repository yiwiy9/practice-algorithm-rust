use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvt: [(Usize1, Usize1, usize); m],
        q: usize,
    }

    let mut dp = vec![vec![INF; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }

    for &(u, v, t) in &uvt {
        dp[u][v] = dp[u][v].min(t);
        dp[v][u] = dp[v][u].min(t);
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
            }
        }
    }

    for _ in 0..q {
        input! {
            k: usize,
            mut b: [Usize1; k],
        }

        let mut ans = INF;

        // next_permutation()
        permutohedron::heap_recursive(&mut b, |b| {
            for bit in 0..(1 << k) {
                let mut cur = 0;

                let mut before_v = 0;
                for i in 0..k {
                    let (u, v, t) = uvt[b[i]];
                    if bit & (1 << i) == 0 {
                        cur += dp[before_v][u] + t;
                        before_v = v;
                    } else {
                        cur += dp[before_v][v] + t;
                        before_v = u;
                    }
                }
                cur += dp[before_v][n - 1];

                ans = ans.min(cur);
            }
        });

        println!("{}", ans);
    }
}
