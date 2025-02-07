use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/abc117/tasks/abc336_e
///
/// 桁DPの問題ということなので解いてみる
/// https://atcoder.jp/contests/abc336/editorial/9055
/// => 割る数を変えると、modは保存しないので、割る数を固定する必要がある
/// => 桁和 s が 1 から 9 * 14 を決め打って、s ごとに桁DPを行う
/// => 余りを使うDPは割る数で一番外側のループを回す
///
/// dp[i][j][k][smaller] := Xの10進数表記のi桁目まで決めて、
///                         桁和がjであり、mod s が kであり、
///                         Nより小さいことが確定しているかどうか（smaller）
///                         のときの良い整数の個数
fn main() {
    input! {
        n: Chars,
    }

    let mut ans = 0_usize;
    for s in 1..=9 * 14 {
        let mut dp = vec![vec![vec![0; 2]; s]; s + 1];
        dp[0][0][0] = 1;
        for &n_i in &n {
            let mut next_dp = vec![vec![vec![0; 2]; s]; s + 1];
            let n_digit = n_i as usize - '0' as usize;
            for j in 0..=s {
                for k in 0..s {
                    for smaller in 0..2 {
                        for d in 0..=9 {
                            if j + d > s {
                                // 桁和が s を超える場合はスキップ
                                break;
                            }
                            if smaller == 0 && d > n_digit {
                                // N より大きい場合はスキップ
                                break;
                            }
                            let next_j = j + d;
                            let next_k = (k * 10 + d) % s;
                            let next_smaller = if smaller == 1 || d < n_digit { 1 } else { 0 };
                            next_dp[next_j][next_k][next_smaller] += dp[j][k][smaller];
                        }
                    }
                }
            }
            dp = next_dp;
        }
        ans += dp[s][0][0] + dp[s][0][1];
    }

    println!("{}", ans);
}
