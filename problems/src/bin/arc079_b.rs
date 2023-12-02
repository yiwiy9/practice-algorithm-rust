use itertools::Itertools;
use proconio::input;

/**
 * https://atcoder.jp/contests/abc068/tasks/arc079_b
 * https://img.atcoder.jp/arc079/editorial.pdf
 * 操作を逆から考える
 */
fn main() {
    input! {
        k: usize,
    }

    let n = 50;
    let offset = k / n;
    let mut a = (0..n).collect::<Vec<usize>>();
    for i in 0..(k % n) {
        a[i] += n + 1;
        for a_j in a.iter_mut() {
            *a_j -= 1;
        }
    }

    println!("{}", n);
    println!("{}", a.iter().map(|a_i| a_i + offset).join(" "));
}
