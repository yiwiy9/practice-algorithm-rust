use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/joi2012ho/tasks/joi2012ho4
/// - 固定するもの:
///   各釘 `(a, b)` に対して、「この釘を頂点として下方向にどこまで輪ゴムで覆われるか」
///   の最大残り辺長だけを持つ。
/// - 欲しい量:
///   `dp[a][b]` := 釘 `(a, b)` を頂点として、下にどこまで塗れるかの最大残り辺長。
///   `dp[a][b] != -1` なら、その釘は 1 本以上の輪ゴムに含まれる。
/// - 必要クエリ:
///   各頂点 `(a, b)` に直接置かれた輪ゴムのうち、最大の `x`。
/// - 単調性 / 境界 / 禁止事項:
///   輪ゴム `(a, b, x)` の影響は `(a+1, b, x-1)` と `(a+1, b+1, x-1)` にしか伝わらない。
///   同じ頂点から始まる輪ゴムは最大の `x` だけ残せば十分なので、
///   `a` の小さい順に 1 回 sweep すればよい。
///
/// AC: 40分
fn main() {
    input! {
        n: usize,
        m: usize,
        abx: [(Usize1,Usize1,i64); m],
    }

    let mut dp = vec![];
    for i in 1..=n {
        let cur = vec![-1; i];
        dp.push(cur);
    }

    for &(a, b, x) in &abx {
        dp[a][b] = dp[a][b].max(x);
    }

    for a in 0..n {
        for b in 0..(a + 1) {
            if dp[a][b] <= 0 {
                continue;
            }
            let next_x = dp[a][b] - 1;
            dp[a + 1][b] = dp[a + 1][b].max(next_x);
            dp[a + 1][b + 1] = dp[a + 1][b + 1].max(next_x);
        }
    }

    println!(
        "{}",
        dp.iter()
            .map(|row| row.iter().filter(|v| **v != -1).count())
            .sum::<usize>()
    );
}
