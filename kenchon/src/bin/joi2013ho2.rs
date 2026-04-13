use proconio::{input, marker::Chars};

const NEG: i32 = -1_000_000_000;

/// https://atcoder.jp/contests/joi2013ho/tasks/joi2013ho2
/// - 固定するもの:
///   `S` の先頭 `i` 文字、`T` の先頭 `j` 文字までを「すでに車庫から出した」とする。
///   そのとき、それらを並べた列の末尾にある「列車になりうる suffix」だけを考える。
/// - 欲しい量:
///   `dp[i][j][0/1]` := 上の suffix のうち、末尾が `O / I` であるものの最大長。
///   先頭は必ず `I`、中身は交互なので、長さそのものではなく
///   「末尾が O か I か」とその最大長だけ持てば十分。
/// - 必要クエリ:
///   次に追加する文字が `I` なら `...O + I` にできるので `dp[*][*][0] + 1`。
///   さらに `I` 単体で新しく始めることもできるので長さ `1` も候補。
///   次が `O` なら `...I + O` にしかできないので `dp[*][*][1] + 1`。
/// - 単調性 / 境界:
///   開始前に捨てる prefix は自由なので、どの `(i, j)` でも「空 suffix 長 0」は常に可能。
///   これを `dp[i][j][0] = 0` として持っておくと、
///   途中の `I` から新しく列車を始める処理まで DP に吸収できる。
fn main() {
    input! {
        m: usize,
        n: usize,
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![[NEG; 2]; n + 1]; m + 1];

    // ここまでに出した車両をすべて待機用レールに送ったとみなせば、
    // どの (i, j) でも長さ 0 の空 suffix は作れる。
    // 「次に I が来たら長さ 1 で始められる」よう、0 側に載せておく。
    for row in &mut dp {
        for cell in row {
            cell[0] = 0;
        }
    }

    let mut ans = 0;
    for i in 0..=m {
        for j in 0..=n {
            ans = ans.max(dp[i][j][1]);

            if i < m {
                let c = s[i];
                if c == 'I' {
                    // `...O` に `I` を足して伸ばすか、ここから長さ 1 で始める。
                    dp[i + 1][j][1] = dp[i + 1][j][1].max(dp[i][j][0] + 1);
                } else if dp[i][j][1] > 0 {
                    // `O` は `...I` の後ろにしか置けない。
                    dp[i + 1][j][0] = dp[i + 1][j][0].max(dp[i][j][1] + 1);
                }
            }

            if j < n {
                let c = t[j];
                if c == 'I' {
                    dp[i][j + 1][1] = dp[i][j + 1][1].max(dp[i][j][0] + 1);
                } else if dp[i][j][1] > 0 {
                    dp[i][j + 1][0] = dp[i][j + 1][0].max(dp[i][j][1] + 1);
                }
            }
        }
    }

    // suffix が `I` で終わっているときだけ、それ自体が完成した列車になる。
    println!("{}", ans);
}
