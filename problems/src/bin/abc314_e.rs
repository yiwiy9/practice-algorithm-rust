use proconio::input;

/**
 * https://atcoder.jp/contests/abc314/tasks/abc314_e
 * https://atcoder.jp/contests/abc314/editorial/6956
 *
 * 0の場合は遷移しないので要注意！
 * 0以外の値が出るまでに支払う金額をCiを各ルーレットごとに計算する
 * 試行回数の期待値は1/p
 * => Ci(0以外) = Ci * Pi / (Pi-0cnt)
 */
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut c = vec![0.0; n];
    let mut p = vec![0.0; n];
    let mut s = vec![vec![]; n];
    for i in 0..n {
        input! {
            c_i: f64,
            p_i: usize,
            s_i: [usize; p_i],
        }

        s[i] = s_i
            .iter()
            .cloned()
            .filter(|&s_j| s_j != 0)
            .collect::<Vec<_>>();

        p[i] = s[i].len() as f64;
        c[i] = c_i * p_i as f64 / (s[i].len() as f64);
    }

    let mut dp = vec![1_000_000.0; m * m + 1];
    for dp_m in dp.iter_mut().skip(m) {
        *dp_m = 0.0;
    }

    for k in (0..m).rev() {
        for i in 0..n {
            let mut dp_i: f64 = 0.0;
            for &s_j in &s[i] {
                dp_i += (dp[k + s_j] + c[i]) / p[i];
            }

            dp[k] = dp[k].min(dp_i);
        }
    }

    println!("{:.8}", dp[0]);
}
