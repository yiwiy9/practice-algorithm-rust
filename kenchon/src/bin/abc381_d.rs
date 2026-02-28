use std::collections::HashSet;

use proconio::input;

/// 尺取り法 / two pointers / sliding window（テンプレ1：right-for, left-while）
/// left=0 で開始
/// for right in 0..n {        // right は 0..n-1 を走る
///     add(a[right]);         // 右端を window に追加
///     while invalid {        // 壊れている間だけ「出して進める」
///         remove(a[left]);   // 左端を window から削除
///         left += 1;         // remove の後に left を進める
///     }
///     update_answer();       // ここで必ず valid（window=[left..=right]）
/// }
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans: usize = 0;
    let mut left = 0;
    let mut set = HashSet::new();
    for right in 0..n / 2 {
        if a[2 * right] != a[2 * right + 1] {
            set.clear();
            left = right + 1;
            continue;
        }

        while set.contains(&a[2 * right]) {
            set.remove(&a[2 * left]);
            left += 1;
        }
        set.insert(a[2 * right]);

        ans = ans.max(right - left + 1);
    }

    let mut left = 1;
    let mut set = HashSet::new();
    for right in 1..=n / 2 {
        if 2 * right >= n || a[2 * right - 1] != a[2 * right] {
            set.clear();
            left = right + 1;
            continue;
        }

        while set.contains(&a[2 * right]) {
            set.remove(&a[2 * left]);
            left += 1;
        }
        set.insert(a[2 * right]);

        ans = ans.max(right - left + 1);
    }

    println!("{}", 2 * ans);
}
