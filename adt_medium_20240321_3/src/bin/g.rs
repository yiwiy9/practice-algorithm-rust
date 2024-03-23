use proconio::input;

/**
 * https://atcoder.jp/contests/adt_medium_20240321_3/tasks/abc270_d
 * https://atcoder.jp/contests/adt_medium_20240321_3/editorial/4850
 *
 * DP[n]= 石がn個残っている状態からゲームを始めたとき、先手が取ることのできる石の個数
 * 遷移は
 * DP[n]=max{Ai+(n−Ai)−DP[n−Ai]}
 *
 * 先手番がAi個の石を取ったとき、最終的に取れる石の個数は、
 * Ai+「石がn−Ai個残っている状態からゲームを始めたとき、後手が取ることのできる石の個数」
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }

    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        for &a_k in &a {
            if i < a_k {
                continue;
            }
            dp[i] = dp[i].max(a_k + (i - a_k) - dp[i - a_k]);
        }
    }

    println!("{}", dp[n]);
}
