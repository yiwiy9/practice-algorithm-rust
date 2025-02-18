use itertools::Itertools;
use proconio::input;

/// https://atcoder.jp/contests/abc393/tasks/abc393_e
/// https://atcoder.jp/contests/abc393/editorial/12243
/// 調和級数を意識して、計算量を抑える
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let max_a = *a.iter().max().unwrap();

    // aにおける各数字の出現回数を数える
    let mut a_cnt = vec![0; max_a + 1];
    for &a_i in &a {
        a_cnt[a_i] += 1;
    }

    // aにおける各数字の倍数の出現回数を数える
    let mut a_cnt_multiple = vec![0; max_a + 1];

    // 調和級数: max_a + max_a/2 + max_a/3 + ... + max_a/max_a ~ O(max_a*log(max_a))
    for d in 1..=max_a {
        for num in (d..=max_a).step_by(d) {
            a_cnt_multiple[d] += a_cnt[num];
        }
    }

    // 各数字が選ばれた場合の最大公約数を求める
    let mut ans = vec![0; max_a + 1];

    // 調和級数: max_a + max_a/2 + max_a/3 + ... + max_a/max_a ~ O(max_a*log(max_a))
    for d in 1..=max_a {
        if a_cnt_multiple[d] < k {
            continue;
        }
        for num in (d..=max_a).step_by(d) {
            // dを約数として持つ数字の最大公約数を更新する
            ans[num] = ans[num].max(d);
        }
    }

    println!("{}", a.iter().map(|&a_i| ans[a_i]).join("\n"));
}
