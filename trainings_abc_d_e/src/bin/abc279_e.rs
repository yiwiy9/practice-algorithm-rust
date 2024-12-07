use itertools::Itertools as _;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc279/tasks/abc279_e
/// https://atcoder.jp/contests/abc279/editorial/5311
/// この問題のように「1つだけ操作を飛ばしたときの結果を求める」という類の問題に対しては、
/// 一般に、前後からの”累積和”を求めるアプローチが有効なことが多くあります。
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    }

    // front_acc[i] = 前から i 回操作を行った後、1'がいる位置
    // 初期は front_acc[0] = 0 (1は位置0にいるとする)
    let mut front_acc = vec![0; m + 1];
    for i in 0..m {
        let prev = front_acc[i];
        if prev == a[i] {
            front_acc[i + 1] = a[i] + 1;
        } else if prev == a[i] + 1 {
            front_acc[i + 1] = a[i];
        } else {
            front_acc[i + 1] = prev;
        }
    }

    // back_acc[i] = 残り (m-i) 回操作を行った結果、i'がいる位置
    let mut back_acc = (0..=n).collect::<Vec<_>>();

    let mut ans = vec![];
    for i in (0..m).rev() {
        ans.push(back_acc[front_acc[i]]);
        back_acc.swap(a[i], a[i] + 1);
    }

    println!("{}", ans.iter().rev().map(|i| i + 1).join("\n"));
}
