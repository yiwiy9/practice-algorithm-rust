use proconio::input;

/// https://atcoder.jp/contests/abc402/tasks/abc402_e
/// https://atcoder.jp/contests/abc402/editorial/12715
/// コンテスト中に解けずに泣いたい問題
///
/// dpの定義をちゃんと自分の言葉で書くべき
/// dp[i][bit] := 現在の所持金がi円で、bitのビットが立っているものがすでに正解している状態のときの **残りの問題から得られる得点の** 期待値の最大値
///
/// つまり、答えは dp[x][0] である
/// → 当たり前のようだが、ここを考えれてなかった（全要素のMAXを答えとしてた）
/// → そして期待値DPなので、遷移は逆順で考える必要がある（正解している状態 → 不正解している状態）
/// → 不正解している状態の期待値は不明なので当然考えられない
///
/// また、問題jを提出したときの遷移は2種類ある
/// 1. 問題jを正解したとき
///    → i → i + c, bit | (1 << j)
/// 2. 問題jを不正解したとき
///    → i → i + c, bit
///
/// 問題jを不正解したときに、bitは変わらず、iは変わるという遷移が本番中に思いつかなかった
/// これは、遷移を iとbit で合わせて考えてしまったから
fn main() {
    input! {
        n: usize,
        x: usize,
        scp: [(f64, usize, f64); n],
    }

    let mut dp = vec![vec![0.0; 1 << n]; x + 1];
    for i in 0..=x {
        for bit in 0..(1 << n) {
            for j in 0..n {
                if bit & (1 << j) != 0 {
                    continue;
                }

                let (s, c, p) = scp[j];
                if i < c {
                    continue;
                }

                let after_i = i - c;
                let after_bit = bit | (1 << j);

                // 正解したとき
                let mut expected = (dp[after_i][after_bit] + s) * p / 100.0;
                // 不正解したとき
                expected += dp[after_i][bit] * (100.0 - p) / 100.0;

                // dp[i][bit] の最大値を更新
                if expected > dp[i][bit] {
                    dp[i][bit] = expected;
                }
            }
        }
    }

    println!("{:.10}", dp[x][0]);
}
