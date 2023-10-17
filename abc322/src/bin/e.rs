use proconio::input;
const INF: usize = 1 << 60;

// P+1進数として扱う
fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
    }

    let mut c = vec![];
    let mut a = vec![];

    for _ in 0..n {
        input! {
            c_i: usize,
            a_i: [usize; k],
        }
        c.push(c_i);

        let mut a_p = 0;
        for a_ij in a_i {
            a_p *= p + 1; // P+1進数として扱う
            a_p += a_ij;
        }
        a.push(a_p);
    }

    let mut max_param = 0;
    for _ in 0..k {
        max_param *= p + 1; // P+1進数として扱う
        max_param += p;
    }

    let mut dp = vec![vec![INF; max_param + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..=max_param {
            if dp[i][j] == INF {
                continue;
            }
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);

            let mut j_mut = j;
            let mut a_mut = a[i];
            let mut x = 1;
            let mut next_j = 0;
            for _ in 0..k {
                next_j += (j_mut % (p + 1) + a_mut % (p + 1)).min(p) * x;
                x *= p + 1; // P+1進数として扱う
                j_mut /= p + 1;
                a_mut /= p + 1;
            }

            dp[i + 1][next_j] = dp[i + 1][next_j].min(dp[i][j] + c[i]);
        }
    }

    println!(
        "{}",
        if dp[n][max_param] == INF {
            -1
        } else {
            dp[n][max_param] as i64
        }
    );
}
