use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/joi2024yo2/tasks/joi2024_yo2_c
/// https://www2.ioi-jp.org/joi/2023/2024-yo2/2024-yo2-t3-review.pdf
/// 考察はできても実装に落とし込めなかった
/// => 何も考えずに累積和を３本取るのではなく、RGBの切れ目を意識して、意味のある累積のみ記録する
/// => そして、累積の中で A も考慮に入れる
/// => 意味のある単位を意識してコーディングする
/// => R - 3 番目まで確定しているとして、R-2,R-1,Rを一回のループで決めてしまう
fn main() {
    input! {
        n: usize,
        s: Chars,
        a: i64,
        b: i64,
        c: i64,
    }

    let mut prefix_sum = [a, 2 * a, 0];
    let mut ans = a.min(b) * n as i64;
    for r in 2..n {
        if s[r - 2] != 'R' {
            prefix_sum[r % 3] += c;
        }
        if s[r - 1] != 'G' {
            prefix_sum[r % 3] += c;
        }
        if s[r] != 'B' {
            prefix_sum[r % 3] += c;
        }

        prefix_sum[r % 3] = prefix_sum[r % 3].min((r + 1) as i64 * a);
        ans = ans.min(prefix_sum[r % 3] + (n - 1 - r) as i64 * b);
    }

    println!("{}", ans);
}
