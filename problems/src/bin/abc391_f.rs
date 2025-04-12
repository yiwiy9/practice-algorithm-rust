use proconio::input;
use std::collections::{BinaryHeap, HashSet};

/// https://atcoder.jp/contests/abc391/tasks/abc391_f
/// 自力AC 失敗
///
/// ポイント
/// - kが小さいので大きい順に全列挙する
/// - ヒープから1件1件取り出して、次の候補をヒープに追加する
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    a.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));
    c.sort_by(|a, b| b.cmp(a));

    let mut set = HashSet::new();

    let mut add =
        |heap: &mut BinaryHeap<(usize, usize, usize, usize)>, i: usize, j: usize, k: usize| {
            if i >= n || j >= n || k >= n {
                return;
            }
            let num = a[i] * b[j] + b[j] * c[k] + c[k] * a[i];
            if set.insert((i, j, k)) {
                heap.push((num, i, j, k));
            }
        };

    let mut heap = BinaryHeap::new();

    add(&mut heap, 0, 0, 0);

    let mut ans = 0;
    for _ in 0..k {
        let (num, i, j, k) = heap.pop().unwrap();
        ans = num;
        add(&mut heap, i + 1, j, k);
        add(&mut heap, i, j + 1, k);
        add(&mut heap, i, j, k + 1);
    }

    println!("{}", ans);
}
