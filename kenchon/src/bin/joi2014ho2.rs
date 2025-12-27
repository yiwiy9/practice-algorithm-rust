use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        m: usize,
        n: usize,
        mut p: [usize; m],
        ce: [(usize,usize); n],
    }

    p.sort_by(|a, b| b.cmp(a));
    let mut p_s = vec![0; m + 1];
    for i in 0..m {
        p_s[i + 1] = p_s[i] + p[i];
    }

    let mut dp = vec![INF; m + 1];
    dp[0] = 0;
    for &(c_i, e_i) in &ce {
        let mut next_dp = vec![INF; m + 1];
        for (j, &v) in dp.iter().enumerate() {
            if v == INF {
                continue;
            }
            next_dp[j] = next_dp[j].min(v);

            let next_j = (j + c_i).min(m);
            next_dp[next_j] = next_dp[next_j].min(v + e_i);
        }
        dp = next_dp;
    }

    let mut box_min = vec![INF; m + 1];
    box_min[m] = dp[m];
    for j in (0..m).rev() {
        box_min[j] = box_min[j + 1].min(dp[j]);
    }

    let mut ans: usize = 0;
    for (j, &v) in box_min.iter().enumerate() {
        if p_s[j] > v {
            ans = ans.max(p_s[j] - v);
        }
    }

    println!("{}", ans);
}
