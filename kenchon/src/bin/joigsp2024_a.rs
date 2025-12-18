use proconio::input;

const INF: usize = 1 << 60;

/// https://img.atcoder.jp/joigsp2024/together.pdf?_gl=1*1lomdyh*_ga*MTA0MzY2NjUyOS4xNjc4ODA3Mzk4*_ga_RC512FD18N*czE3NjYwNzEwMjkkbzEwNzgkZzEkdDE3NjYwNzM1MjAkajIwJGwwJGgw
/// https://www2.ioi-jp.org/joig-camp/2024/2024-sp-tasks/contest1/together-review.pdf
/// いっしょ (Together)
///
/// dp[i] : i 匹目まで組んだときの移動距離の総和の最小値
/// ((↑ さらに次元追加して決まった/決まってないを管理しようとして遷移がどうにもならなくなった))
///
/// 2匹をまとめるパターンと3匹をまとめるパターンしかない
/// => ここまでは考察できていたが、DPの定義が下手
/// => DPは部分問題を解いている、つまりdp[i]は後ろの状態に依らず、常に決まっているように設計する必要がある
///
/// 上記を踏まえた上で、ここまで（例えばi-2, i-3）決まっているとして、どう決めると次の状態（i）にいくかを考える
fn main() {
    input! {
        n: usize,
        mut x: [usize; n],
    }

    x.sort();

    let mut dp = vec![INF; n];
    dp[1] = x[1] - x[0];
    if n > 2 {
        dp[2] = x[2] - x[0];
    }
    for i in 3..n {
        dp[i] = dp[i].min(dp[i - 2] + x[i] - x[i - 1]); // i番目をi-1番目に移動する
        dp[i] = dp[i].min(dp[i - 3] + x[i] - x[i - 2]); // i番目とi-2番目をi-1番目に移動する
    }

    println!("{}", dp[n - 1]);
}
