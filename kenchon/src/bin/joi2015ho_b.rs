use proconio::input;

/// https://atcoder.jp/contests/joi2015ho/tasks/joi2015ho_b
/// https://www2.ioi-jp.org/joi/2014/2015-ho/2015-ho-t2-review.pdf
/// DPっぽいなと思ったが、JOIとIOIで違う行動をするのを式に落とし込めなかった
/// => 区間DP（lenを1から大きくしていくDP）
/// => どっちのターンかは区間の長さの偶奇で決まる
///
/// JOIくんのターンから始まる場合
///   dp[i][j] = max(dp[i+1][j]+A[i],dp[i][j-1]+A[j]);
///
/// IOIちゃんのターンから始まる場合
///   dp[i][j] = dp[i+1][j] (A[i]>A[j]のとき)
///   dp[i][j] = dp[i][j-1] (A[i]<A[j]のとき)
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = a.clone();
    b.extend_from_slice(&a);

    let m = b.len();
    let mut dp = vec![vec![0usize; m]; m];

    for len in 1..=n {
        for l in 0..=m - len {
            let r = l + len - 1;

            if len % 2 == n % 2 {
                // JOI の手番
                if len == 1 {
                    dp[l][r] = b[l];
                } else {
                    dp[l][r] = (dp[l + 1][r] + b[l]).max(dp[l][r - 1] + b[r]);
                }
            } else {
                // IOI の手番
                if len == 1 {
                    dp[l][r] = 0;
                } else if b[l] > b[r] {
                    dp[l][r] = dp[l + 1][r];
                } else {
                    dp[l][r] = dp[l][r - 1];
                }
            }
        }
    }

    println!("{}", (0..n).map(|i| dp[i][i + n - 1]).max().unwrap());
}
