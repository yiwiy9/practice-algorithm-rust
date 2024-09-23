use itertools::Itertools;
use proconio::input;

/**
 * https://atcoder.jp/contests/abc359/tasks/abc359_e
 * https://atcoder.jp/contests/abc359/editorial/10262
 * ABC372_D の類題
 */
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = vec![0; n + 1];
    ans[0] = 1;

    let mut stack = vec![];

    for i in 1..=n {
        ans[i] = ans[i - 1];

        let mut cnt = 1;
        while let Some(&(h_j, cnt_j)) = stack.last() {
            if h_j > h[i - 1] {
                break;
            }
            cnt += cnt_j;
            ans[i] -= h_j * cnt_j;
            stack.pop();
        }

        ans[i] += h[i - 1] * cnt;
        stack.push((h[i - 1], cnt));
    }

    println!("{}", ans.iter().skip(1).join(" "));
}
