use proconio::{input, marker::Chars};
use superslice::*;

const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/joi2020ho/tasks/joi2020ho_b
/// https://drken1215.hatenablog.com/entry/2020/11/28/020600
/// 尺取りを思いついたがWAを取れなかった
/// => 思いついた解法をそのまま実装するのではなく、一段抽象化して考える勇気
/// => 特に尺取りは絶対バグるので二分探索を模索する癖つける
///
/// 左端のJを固定して、二分探索で解ける
fn main() {
    input! {
        _: usize,
        k: usize,
        s: Chars,
    }

    let mut j_indexes = vec![];
    let mut o_indexes = vec![];
    let mut i_indexes = vec![];
    for (i, &c) in s.iter().enumerate() {
        match c {
            'J' => j_indexes.push(i),
            'O' => o_indexes.push(i),
            'I' => i_indexes.push(i),
            _ => unreachable!(),
        }
    }

    if j_indexes.len() < k || o_indexes.len() < k || i_indexes.len() < k {
        println!("-1");
        return;
    }

    let mut ans = INF;
    for i in 0..=j_indexes.len() - k {
        let left_j_index = j_indexes[i];
        let right_j_index = j_indexes[i + k - 1];

        let j = o_indexes.lower_bound(&right_j_index);
        if j + k > o_indexes.len() {
            break;
        }
        let right_o_index = o_indexes[j + k - 1];

        let l = i_indexes.lower_bound(&right_o_index);
        if l + k > i_indexes.len() {
            break;
        }
        let right_i_index = i_indexes[l + k - 1];

        ans = ans.min(right_i_index - left_j_index + 1 - 3 * k);
    }

    println!("{}", if ans == INF { -1 } else { ans as i64 });
}
