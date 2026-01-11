use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use superslice::*;

/// https://atcoder.jp/contests/joigsp2025/tasks/joigsp2025_f
/// https://www2.ioi-jp.org/joig-camp/2025/2025-sp-tasks/contest2/emerge-review.pdf
/// https://www2.ioi-jp.org/joig-camp/2025/2025-sp-tasks/contest2/emerge-sample.cpp
///
/// 実はHでループができるのでシミュレーションできる
/// => あとは陸地ごとに隣接の陸地への辺を合成までの回数を重みに引けば、クラスカル法に落とせる
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rc: [(Usize1, Usize1); n],
    }

    let mut c_r_vec = vec![vec![]; w];
    for (i, &(r, c)) in rc.iter().enumerate() {
        c_r_vec[c].push((r, i));
    }
    c_r_vec.iter_mut().for_each(|r_vec| r_vec.sort());

    // 各陸地ごとに合成する可能性のある陸地は隣接の6点のみである
    // 左2点と下1点の3点を押さえれば網羅できる
    let mut edges = vec![vec![]; h];
    for (i, &(r, c)) in rc.iter().enumerate() {
        // 下
        let upper_r_idx = c_r_vec[c].upper_bound(&(r, n));
        if upper_r_idx < c_r_vec[c].len() {
            let (upper_r, j) = c_r_vec[c][upper_r_idx];
            edges[upper_r - r - 1].push((i, j));
        }

        // 左
        if c == 0 {
            continue;
        }

        // 左下
        let left_lower_r_idx = c_r_vec[c - 1].lower_bound(&(r, 0));
        if left_lower_r_idx < c_r_vec[c - 1].len() {
            let (left_lower_r, j) = c_r_vec[c - 1][left_lower_r_idx];
            edges[left_lower_r - r].push((i, j));
        }

        // 左上
        let left_upper_r_idx = c_r_vec[c - 1].upper_bound(&(r, n));
        if left_upper_r_idx > 0 {
            let (left_upper_r, j) = c_r_vec[c - 1][left_upper_r_idx - 1];
            edges[r - left_upper_r].push((i, j));
        }
    }

    let mut dsu = Dsu::new(n);
    for (t, edges_t) in edges.iter().enumerate() {
        for &(i, j) in edges_t {
            dsu.merge(i, j);
        }

        if dsu.size(0) == n {
            println!("{}", t);
            return;
        }
    }

    println!("-1");
}
