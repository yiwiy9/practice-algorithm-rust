use proconio::{input, marker::Usize1};

const INF: usize = 1 << 60;

/// ビットDP(bit DP)
/// https://atcoder.jp/contests/joi2017yo/tasks/joi2017yo_d
/// https://blog.hamayanhamayan.com/entry/2019/10/27/233512
/// 手も足も出なかった
/// => こういうときは大体 bit DP
///
/// 種類の順番は最大20!あって、不可能だが、bitDPではN!を2^Nに変換して解決するのに最適な方針である。
///     dp[msk] := 既に順番が決まった種類の集合をmskとしたときに、取り出す必要のある個数の最小値
/// 順番が12であろうと、21であろうと、その時点で結果として決まっている個数は一緒なので、
/// 順番の3つ目以降には影響しないため、状態をまとめられるというのがモチベーション。
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; n],
    }

    let mut s = vec![vec![0; n + 1]; m];
    for j in 0..m {
        for i in 0..n {
            s[j][i + 1] = s[j][i] + if a[i] == j { 1 } else { 0 };
        }
    }

    let mut dp = vec![INF; 1 << m];
    dp[0] = 0;
    for bit in 0..(1 << m) {
        let mut done = 0;
        for j in 0..m {
            if bit & (1 << j) != 0 {
                done += s[j][n];
            }
        }

        for next_j in 0..m {
            if bit & (1 << next_j) != 0 {
                continue;
            }

            let next_j_total = s[next_j][n];
            let next_j_in_range = s[next_j][done + next_j_total] - s[next_j][done];
            let next_cost = dp[bit] + next_j_total - next_j_in_range;

            let next_bit = bit | (1 << next_j);
            dp[next_bit] = dp[next_bit].min(next_cost);
        }
    }

    println!("{}", dp[(1 << m) - 1]);
}
