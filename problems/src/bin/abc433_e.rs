use itertools::Itertools;
use proconio::input;

/// https://atcoder.jp/contests/abc433/tasks/abc433_e
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   値を `N*M` から大きい順に置く話にする
///   持つのは active な行、active な列、active×active の未使用マスだけ
/// - それについて、何が分かれば答えになる？
///   各値 `v` を置くべき場所が毎回 1 か所以上見つかれば答えを構成できる
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   行や列の中の細かい順序は持たなくてよい
///   大きい値ほど置ける場所が少ないので、大きい順に決めれば矛盾を先に検出できる
///   マス `(i,j)` は、その行 `i` と列 `j` の最大値が両方処理済みになった瞬間にだけ候補になる
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   `row_of[v]`, `col_of[v]` を作って `v` を大きい順に処理する
///   行にも列にもあるなら交点、行だけなら active な列、列だけなら active な行に置く
///   どちらにもないなら active×active の空きマスから 1 つ取って置く
///   新しく active になった行/列と既存 active の組を空きマス集合へ追加する
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            x: [usize; n],
            y: [usize; m],
        }

        match solve(n, m, &x, &y) {
            Some(ans) => {
                println!("Yes");
                println!("{}", ans.iter().map(|row| row.iter().join(" ")).join("\n"));
            }
            None => {
                println!("No");
            }
        }
    }
}

fn solve(n: usize, m: usize, x: &[usize], y: &[usize]) -> Option<Vec<Vec<usize>>> {
    let nm = n * m;
    let mut row_of = vec![None; nm + 1];
    let mut col_of = vec![None; nm + 1];

    // 各値を、どの行/列の最大値として使うかを記録する。
    for (i, &x_i) in x.iter().enumerate() {
        if row_of[x_i].is_some() {
            return None;
        }
        row_of[x_i] = Some(i);
    }
    for (j, &y_j) in y.iter().enumerate() {
        if col_of[y_j].is_some() {
            return None;
        }
        col_of[y_j] = Some(j);
    }

    let mut ans = vec![vec![0; m]; n];
    let mut active_rows = vec![];
    let mut active_cols = vec![];
    // active×active になっていて、まだ小さい値を置ける空きマスの集合。
    // 各マスは「行と列の両方が active になった瞬間」に高々 1 回だけここへ入るので、
    // 追加回数の総和は O(N*M) で間に合う。
    let mut avail = vec![];

    // 大きい値から置くと、行最大・列最大の制約を先に確定できる。
    for v in (1..=nm).rev() {
        match (row_of[v], col_of[v]) {
            (Some(r), Some(c)) => {
                // 行 r と列 c の両方の最大値なので、交点に置く。
                ans[r][c] = v;
                for &cc in &active_cols {
                    avail.push((r, cc));
                }
                for &rr in &active_rows {
                    avail.push((rr, c));
                }
                active_rows.push(r);
                active_cols.push(c);
            }
            (Some(r), None) => {
                // 行 r が新しく active になるので、既存の active な列のどこかに置く。
                let &c0 = active_cols.first()?;
                ans[r][c0] = v;
                for &cc in &active_cols {
                    if cc != c0 {
                        avail.push((r, cc));
                    }
                }
                active_rows.push(r);
            }
            (None, Some(c)) => {
                // 列 c が新しく active になる場合も同様。
                let &r0 = active_rows.first()?;
                ans[r0][c] = v;
                for &rr in &active_rows {
                    if rr != r0 {
                        avail.push((rr, c));
                    }
                }
                active_cols.push(c);
            }
            (None, None) => {
                // 小さい値は、行最大・列最大が確定済みの空きマスへ入れればよい。
                let (r, c) = avail.pop()?;
                ans[r][c] = v;
            }
        }
    }

    Some(ans)
}
