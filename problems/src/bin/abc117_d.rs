use proconio::input;
const MAX_DIGIT: usize = 40; // 2^40 = 1_099_511_627_776

/// https://atcoder.jp/contests/abc117/tasks/abc117_d
///
/// 桁DPの問題ということなので解いてみる
/// https://drken1215.hatenablog.com/entry/2019/02/04/013700
/// dp[i][smaller] := Xの2進数表記のi桁目まで決めて、kより小さいことが確定しているかどうか（smaller）のfの最大値
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut dp = vec![vec![-1; 2]; MAX_DIGIT + 1];
    dp[0][0] = 0;
    for d in 0..MAX_DIGIT {
        let mask = 1 << (MAX_DIGIT - d - 1);

        // A で元々 d 桁目にビットが立っているものの個数
        let mut num_of_bits = 0;
        for &a_i in &a {
            if (a_i & mask) != 0 {
                num_of_bits += 1;
            }
        }

        // X の d 桁目を 0, 1 にしたときのスコア
        let zero_score = mask * num_of_bits;
        let one_score = mask * (n as i64 - num_of_bits);

        // smaller -> smaller
        // 数字はなんでも良いので、大きい方を選ぶ
        if dp[d][1] != -1 {
            dp[d + 1][1] = dp[d + 1][1].max(dp[d][1] + zero_score.max(one_score));
        }

        // not smaller -> smaller
        // K の d 桁目が 1 だったら、X の d 桁目は 0 にする
        if dp[d][0] != -1 && k & mask != 0 {
            dp[d + 1][1] = dp[d + 1][1].max(dp[d][0] + zero_score);
        }

        // not smaller -> not smaller
        // K の d 桁目と X の d 桁目を同じにする
        if dp[d][0] != -1 {
            if k & mask != 0 {
                dp[d + 1][0] = dp[d + 1][0].max(dp[d][0] + one_score);
            } else {
                dp[d + 1][0] = dp[d + 1][0].max(dp[d][0] + zero_score);
            }
        }
    }

    println!("{}", dp[MAX_DIGIT][0].max(dp[MAX_DIGIT][1]));
}
