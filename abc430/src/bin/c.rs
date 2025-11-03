use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/abc430/tasks/abc430_c
/// https://atcoder.jp/contests/abc430/editorial/14299
///
/// 尺取りしか思い付かなかった上に実装に落とし込めなかった
/// => 条件を満たす範囲を発見したときにaは右にbは左に伸びてしまうなと思い、手も足も出ず
/// => こういうときは何かを固定する（当日は a起点 or  b起点 みたいな感じで考えてたから右にも左にもってなった）
/// => **左端を固定**
/// => そうすると、各ループで求めるのは右端の場合の数のみでよくなる
/// => bの条件を満たす最大の位置 - aの条件を満たす最小の位置
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }

    let mut ans = 0;
    let mut a_cnt = 0;
    let mut b_cnt = 0;
    let mut a_right = 0;
    let mut b_right = 0;
    for left in 0..n {
        while a_right < n && a_cnt < a {
            if s[a_right] == 'a' {
                a_cnt += 1;
            }
            a_right += 1;
        }

        while b_right < n && b_cnt < b {
            if s[b_right] == 'b' {
                b_cnt += 1;
            }
            b_right += 1;
        }
        if b_right == n && b_cnt < b {
            b_right += 1;
        }

        if a_cnt >= a && b_right > a_right {
            ans += b_right - a_right;
        }

        if s[left] == 'a' {
            a_cnt -= 1;
        } else {
            b_cnt -= 1;
        }
    }

    println!("{}", ans);
}
