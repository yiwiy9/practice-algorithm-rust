use proconio::input;

/// 観察
/// - 豆は前にしか移動できないので、後ろから見ていく
/// - 逆から見ると良さそう
///   - 豆が全て茶碗0にある状態からスタートして、入力の形に持っていくための最小コストを求める
/// - 幅優先探索で一見行けそうだが、それだと複数回カウントするパスが存在する
///
/// 1 2 2 1
/// 0 1 0 1
///
///
/// 解答
/// https://atcoder.jp/contests/abc404/tasks/abc404_e
/// https://atcoder.jp/contests/abc404/editorial/12866
///
/// - 後ろから見ていくで良い
/// - 自分の前の豆の位置まで最小のコストで移動するを茶碗0まで繰り返す
fn main() {
    input! {
        n: usize,
        c: [usize; n-1],
        a: [usize; n-1],
    }

    let beans = a
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x > 0)
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();

    let mut ans = 0;
    for k in (0..beans.len()).rev() {
        let (pre, cur) = if k == 0 {
            (0, beans[k])
        } else {
            (beans[k - 1], beans[k])
        };

        let mut l = cur;
        while pre < l {
            ans += 1;
            let mut nl = l;
            for i in l..=cur {
                nl = nl.min(i - c[i - 1]);
            }
            l = nl;
        }
    }

    println!("{}", ans);
}
