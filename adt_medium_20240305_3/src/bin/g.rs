use proconio::input;

/**
 * https://atcoder.jp/contests/adt_medium_20240305_3/tasks/abc318_d
 * https://atcoder.jp/contests/adt_medium_20240305_3/editorial/7066
 *
 * bit DP
 * dp[b] := b のうち立っているビットを i1,i2,…,ik としたとき、頂点 i1,i2,…,ik については
 *         その頂点を端点とする辺を選んでいる、またはその頂点を端点とする辺は選ばないことを決定した際の、選んだ辺の重みの総和の最大値
 */
fn main() {
    input! {
        n: usize,
    }

    let mut d = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        for j in i + 1..n {
            input! {
                d_ij: usize,
            }
            d[i][j] = d_ij;
            d[j][i] = d_ij;
        }
    }

    let mut dp = vec![0; 1 << n];
    for b in 0..1 << n {
        for i in 0..n {
            for j in i + 1..n {
                if (b >> i & 1) != 1 && (b >> j & 1) != 1 {
                    let nb = b | 1 << i | 1 << j;
                    dp[nb] = dp[nb].max(dp[b] + d[i][j]);
                }
            }
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}
