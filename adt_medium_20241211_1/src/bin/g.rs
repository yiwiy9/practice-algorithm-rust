use proconio::{input, marker::Chars};
const INF: usize = 1 << 30;

fn main() {
    input! {
        t: Chars,
        n: usize,
    }

    let mut s = vec![];
    for i in 0..n {
        input! {
            a_i: usize,
            s_i: [Chars; a_i],
        }
        s.push(s_i);
    }

    // dp[i][j]: t の j 文字目までを作るのに必要な文字列の最小個数
    let mut dp = vec![INF; t.len() + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut next_dp = vec![INF; t.len() + 1];
        for j in 0..=t.len() {
            if dp[j] == INF {
                continue;
            }
            next_dp[j] = next_dp[j].min(dp[j]);
            for s_i in &s[i] {
                if s_i.len() > t.len() - j {
                    continue;
                }
                let mut ok = true;
                for k in 0..s_i.len() {
                    if s_i[k] != t[j + k] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    next_dp[j + s_i.len()] = next_dp[j + s_i.len()].min(dp[j] + 1);
                }
            }
        }
        dp = next_dp;
    }

    println!(
        "{}",
        if dp[t.len()] == INF {
            -1
        } else {
            dp[t.len()] as i32
        }
    );
}
