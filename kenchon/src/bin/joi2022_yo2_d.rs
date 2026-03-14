use proconio::input;

/// https://atcoder.jp/contests/joi2022yo2/tasks/joi2022_yo2_d
/// https://www2.ioi-jp.org/joi/2021/2022-yo2/2022-yo2-t4-review.html
/// dp[i][j]:
/// （誤）i番目の要素を選ぶとき、最後に選んだ要素が i - j 番目であるときのスコアの最大値
/// （正）最後に選んだ 2 個のうち、最後が i 番目で、その 1 つ前が「0..=j のどこか」にある場合の最大値
///
/// → 自分が考えた方針だと、最後に選んだ要素が十分遠いのが最適のときに取りこぼしている
///
///
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }

    let mut dp = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..i {
            // prefix max:
            // dp[i][j] を「1つ前が j 以下でよい」と解釈できるようにする
            if j > 0 {
                dp[i][j] = dp[i][j - 1];
            }

            // 新しく 2 個だけ選ぶケース: (j, i)
            dp[i][j] = dp[i][j].max(a[j] + a[i]);

            // 以前の状態 dp[j][x] から i を追加するケース
            //
            // 条件:
            //   長さ K の区間に 3 個入ってはいけない
            //   <=> x, j, i を選ぶなら i - x >= K が必要
            //   <=> x <= i - K
            //
            // dp[j][t] は「x <= t の中で最大」を持っているので、
            // t = min(j-1, i-k) を使えば、
            //   x < j かつ x <= i-k
            // を満たす x の中で最大を取れる。
            if i >= k && j > 0 {
                let limit = (i - k).min(j - 1);
                dp[i][j] = dp[i][j].max(dp[j][limit] + a[i]);
            }
        }
    }

    println!(
        "{}",
        dp.iter()
            .map(|dp_i| dp_i.iter().max().unwrap())
            .max()
            .unwrap()
    );
}
