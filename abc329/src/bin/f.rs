use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

/**
 * https://atcoder.jp/contests/abc329/tasks/abc329_f
 * マージテク
 *
 * appendはO(n + m)
 * extendはO(m)
 */
fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
    }

    let mut box_vec = vec![BTreeSet::new(); n];
    for (i, &c_i) in c.iter().enumerate() {
        box_vec[i].insert(c_i);
    }

    for _ in 0..q {
        input! {
            a: Usize1,
            b: Usize1,
        }

        // Rustでは&mut Tの値にはそのまま(T) -> Tの関数を適応することができないので、
        // std::mem::take()を使い、O(1)で取り出せる。元のものは空になる
        let mut box_a = std::mem::take(&mut box_vec[a]);
        let mut box_b = std::mem::take(&mut box_vec[b]);

        // マージテク: 小さい方がループするようにswap（O(1)）する
        if box_a.len() > box_b.len() {
            std::mem::swap(&mut box_a, &mut box_b);
        }

        box_b.extend(box_a);
        box_vec[b] = box_b;

        println!("{}", box_vec[b].len());
    }
}
