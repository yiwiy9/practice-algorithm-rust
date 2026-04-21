use itertools::Itertools;
use proconio::input;

const INF: i64 = 1 << 60;

/// https://atcoder.jp/contests/joisc2008/tasks/joisc2008_cheating
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   答え候補 `d_max` を固定した判定問題にして、小さい方の座標から「距離が `d_max` 以内となる座標」をまとめるだけ。
/// - それについて、何が分かれば答えになる？
///   座標群の数がx方向とy方向合わせて、n以下となる `d_max` の最小値を求める
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   x方向とy方向は独立に考えてよい。全ての監視装置について d=d_max でよい。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   x, yそれぞれの独立したソート済み配列を用意し、`d_max` の二分探索を行う
///   判定問題は、「配列を前から見ていき、+d_max以内に収まる点は除外していき、残った点の個数の和がn以下である」
///   n以下であるならば、true → right = mid
///   そうでないならば、false → left = mid
fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(i64,i64); m],
    }

    let mut x = xy.iter().map(|&(x, _)| x).collect_vec();
    let mut y = xy.iter().map(|&(_, y)| y).collect_vec();
    x.sort();
    y.sort();

    let mut left = -1;
    let mut right = INF;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut cnt = 0;

        let mut before_plus_d = -1;
        for &x_i in &x {
            if x_i <= before_plus_d {
                continue;
            }
            cnt += 1;
            before_plus_d = x_i + mid;
        }

        before_plus_d = -1;
        for &y_i in &y {
            if y_i <= before_plus_d {
                continue;
            }
            cnt += 1;
            before_plus_d = y_i + mid;
        }

        if cnt <= n {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
