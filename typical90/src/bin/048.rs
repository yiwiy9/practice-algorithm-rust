use proconio::input;

/**
 * https://atcoder.jp/contests/typical90/tasks/typical90_av
 * https://twitter.com/e869120/status/1396960059796582400/photo/1
 *
 * [B1...Bn, A1-B1...An-Bn]: 1分で得られる得点の配列
 * Ai/2 < Bi < Ai より、Ai-Biを選ぶ時にはすでにBiは選ばれるはずなので
 * この配列をソートして大きい順にK個足し上げる
**/
fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut points_in_a_minute = vec![];
    for (a, b) in ab {
        points_in_a_minute.push(b);
        points_in_a_minute.push(a - b);
    }

    points_in_a_minute.sort();

    let ans: usize = points_in_a_minute.iter().rev().take(k).sum();
    println!("{}", ans);
}
