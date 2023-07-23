use proconio::{input, marker::Usize1};

/**
 * https://atcoder.jp/contests/abc311/tasks/abc311_e
 * 該当のマスを右下の角とした正方形の辺の長さの最大値を足し上げる
 * → １つ上・左・左上それぞれのマスに対する正方形の辺の長さの最小値 + 1 が該当マスの正方形の辺の長さ
 *
 * dp[i][j] = min(dp[i][j-1], dp[i-1][j], dp[i-1][j-1]) + 1
 *
 * 参考：最大正方形の面積の求め方を知ってますか？　2次元の動的計画法（貰う/配る）をビジュアライズしてみました！
 * https://qiita.com/H20/items/884551b4965739176afc
 */
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(Usize1,Usize1); n],
    }

    let mut defected = vec![vec![false; w]; h];
    for &(a, b) in &ab {
        defected[a][b] = true;
    }

    let mut dp = vec![vec![0_usize; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            if defected[i - 1][j - 1] {
                continue;
            }
            let mut min_squares = dp[i][j - 1].min(dp[i - 1][j]);
            min_squares = min_squares.min(dp[i - 1][j - 1]);
            dp[i][j] = min_squares + 1;
        }
    }

    println!(
        "{}",
        dp.iter()
            .map(|row| row.iter().sum::<usize>())
            .sum::<usize>()
    );
}
