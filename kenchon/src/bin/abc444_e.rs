use proconio::input;
use std::collections::BTreeSet;

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
        d: usize,
        a: [usize; n],
    }

    let mut ans: usize = 0;
    let mut left = 0;
    let mut set = BTreeSet::new();
    for right in 0..n {
        let new = a[right];
        loop {
            let mut valid = true;
            if let Some(&right_left) = set.range(..new).next_back() {
                if new.saturating_sub(d) < right_left {
                    valid = false;
                }
            }
            if let Some(&right_right) = set.range(new..).next() {
                if new + d > right_right {
                    valid = false;
                }
            }

            if valid {
                break;
            }

            set.remove(&a[left]);
            left += 1;
        }

        set.insert(new);
        ans += right - left + 1;
    }

    println!("{}", ans);
}
