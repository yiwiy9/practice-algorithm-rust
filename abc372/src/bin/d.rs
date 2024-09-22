use itertools::Itertools;
use proconio::input;

/**
 * https://atcoder.jp/contests/abc372/tasks/abc372_d
 * https://atcoder.jp/contests/abc372/editorial/10975
 *
 * 各 i に対して条件を満たす j を昇順に並べたとき、Hj1 < Hj2 < ... が成り立つ
 * このことに気づけば、i を後ろから見ていって、Hj1, Hj2, ... と Hi が入るところまで遡ればよい
 */
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = vec![0; n];
    let mut stack = vec![];
    for i in (0..n - 1).rev() {
        while let Some(&j) = stack.last() {
            if h[j] > h[i + 1] {
                break;
            }
            stack.pop();
        }
        stack.push(i + 1);
        ans[i] = stack.len();
    }

    println!("{}", ans.iter().join(" "));
}
