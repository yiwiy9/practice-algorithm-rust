use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc186/tasks/abc186_f
/// https://drken1215.hatenablog.com/entry/2020/12/20/111600
/// 「横移動 + 縦移動」で行けるところをすべて求めるのは簡単だし、
/// 「縦移動 + 横移動」で行けるところをすべて求めるのも簡単。
/// しかし、両方の方法で行けるマスの扱いが難しい。
///
/// 実装方法が思いつかんかった
/// => 具体的には W マスからなる BIT を用意して、
///    各マスは「すでに壁に遮られなら 1」「まだ壁に遮られていないなら 0」を表すようにする。
fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut row_vec = vec![vec![w]; h];
    let mut col_vec = vec![vec![h]; w];
    for &(x, y) in &xy {
        row_vec[x].push(y);
        col_vec[y].push(x);
    }

    row_vec.iter_mut().for_each(|v| v.sort());
    col_vec.iter_mut().for_each(|v| v.sort());

    // 横 → 縦
    let mut ans: usize = 0;
    for j in 0..row_vec[0][0] {
        ans += col_vec[j][0];
    }

    // 縦 → 横のうち、横 → 縦で行けないところ
    let mut ft_col_cnt = FenwickTree::new(w + 1, 0);
    for j in row_vec[0][0]..w {
        // 1行目の横移動で到達できない箇所を記録
        ft_col_cnt.add(j, 1);
    }
    for i in 1..col_vec[0][0] {
        ans += ft_col_cnt.sum(0..row_vec[i][0]);
        for &j in &row_vec[i] {
            if ft_col_cnt.sum(j..j + 1) == 0 {
                ft_col_cnt.add(j, 1);
            }
        }
    }

    println!("{}", ans);
}
