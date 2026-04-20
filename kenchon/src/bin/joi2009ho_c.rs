use itertools::Itertools;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_c
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   各横棒の端点についてスタートが対応する位置とゴールが対応する位置を持てば良い
/// - それについて、何が分かれば答えになる？
///   横棒を消さない状態での得点を求め、各横棒毎にループし、消した場合の増減で最小値を計算する
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   あみだくじの横棒を消す = その横棒の端点の結果を入れ替える のため横棒ごとに見るだけで良い
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   あみだくじの横棒の高さが小さい順に対応する縦棒を記録し、swapしていく
///   ゴール時点での対応する縦棒1~kの得点の和(total → ans)を取る
///   再度横棒でループし、端点の縦棒が (1..=k)と(k+1..=n)の場合のみ、
///   total - s[start_pos[(1..=k)の位置]] + s[start_pos[(k+1..=n)の位置]] → MIN(anx) を 取る
///
/// AC: 43分
fn main() {
    input! {
        n: usize,
        m: usize,
        _h: usize,
        k: usize,
        s: [usize; n],
        mut ab: [(Usize1, usize); m],
    }

    ab.sort_by(|a, b| a.1.cmp(&b.1));

    let mut start_pos = (0..n).collect_vec();
    let mut horizontal_lines = vec![];
    for &(a, _) in &ab {
        horizontal_lines.push((start_pos[a], start_pos[a + 1]));
        start_pos.swap(a, a + 1);
    }

    let mut start_pos_scores = vec![0; n];
    for (i, &start_pos_i) in start_pos.iter().enumerate() {
        start_pos_scores[start_pos_i] = s[i];
    }

    let total: usize = start_pos_scores.iter().take(k).sum();

    let mut ans = total;
    for &(start_pos_a, start_pos_b) in &horizontal_lines {
        let small_start_pos = start_pos_a.min(start_pos_b);
        let large_start_pos = start_pos_a.max(start_pos_b);
        if large_start_pos < k || small_start_pos >= k {
            continue;
        }

        ans =
            ans.min(total + start_pos_scores[large_start_pos] - start_pos_scores[small_start_pos]);
    }

    println!("{}", ans);
}
