use proconio::input;
use superslice::Ext;
const INF: i64 = 1 << 60;

/// https://atcoder.jp/contests/joi2014ho/tasks/joi2014ho3
/// - 固定するもの: X を固定して判定
/// - 欲しい量: 始点 i から 3 回飛んで 1 周以内に収まるか
/// - 必要クエリ: next[i] = 和が X 以上になる最小右端
/// - 単調性 / 境界: X が大きいほど判定は厳しい
///
/// AC: 44分
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut s = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        s[i + 1] = s[i] + a[i % n];
    }

    let mut left = 0;
    let mut right = INF;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut ok = false;
        for start_i in 0..n {
            let next_start_i = s.lower_bound(&(s[start_i] + mid));
            if next_start_i >= s.len() {
                continue;
            }

            let next_next_start_i = s.lower_bound(&(s[next_start_i] + mid));
            if next_next_start_i >= s.len() || n + start_i <= next_next_start_i {
                continue;
            }

            if s[n + start_i] - s[next_next_start_i] >= mid {
                ok = true;
                break;
            }
        }

        if ok {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
