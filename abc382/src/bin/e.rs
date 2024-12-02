use proconio::input;

/// https://atcoder.jp/contests/abc382/tasks/abc382_e
/// https://atcoder.jp/contests/abc382/editorial/11483
/// 期待値 DP
/// 冷静にdp_iからの遷移を紙に書いて考察すれば解ける
/// 本番では焦って、一回の遷移で期待値をレアカード枚数分進めてたが、
/// 普通に考えて、期待値はパックを開ける回数なので、1しか増えない
fn main() {
    input! {
        n: usize,
        x: usize,
        p: [f64; n],
    }

    let mut dp_p = vec![0.0; n + 1];
    dp_p[0] = 1.0;
    for i in 0..n {
        let mut new_dp_p = vec![0.0; n + 1];
        for j in 0..n {
            new_dp_p[j + 1] += dp_p[j] * p[i] / 100.0;
            new_dp_p[j] += dp_p[j] * ((100.0 - p[i]) / 100.0);
        }
        dp_p = new_dp_p;
    }

    let mut dp_e = vec![0.0; x + 1];
    for j in (0..x).rev() {
        for i in 1..=n {
            if j + i < x {
                // dp_e[j] += dp_p[i] / (1.0 - dp_p[0]) * (1.0 + dp_e[j + i]); // ここが間違っていた。dp_p[0]を足していないので確率の合計が1にならない
                dp_e[j] += dp_p[i] * dp_e[j + i];
            } else {
                // dp_e[j] += dp_p[i] / (1.0 - dp_p[0]) * (1.0 + dp_e[x]);
            };
        }
        dp_e[j] += 1.0;
        dp_e[j] /= 1.0 - dp_p[0];
    }

    println!("{:.10}", dp_e[0]);
}
