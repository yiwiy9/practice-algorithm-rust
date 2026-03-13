use proconio::input;
use std::collections::BTreeMap;

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
        x: usize,
        y: usize,
        a: [usize; n],
    }

    let mut ans: usize = 0;
    let mut last_x_pos = n;
    let mut last_y_pos = n;
    let mut left = 0;
    let mut cnt_map: BTreeMap<usize, usize> = BTreeMap::new();
    for right in 0..n {
        *cnt_map.entry(a[right]).or_default() += 1;
        if a[right] == x {
            last_x_pos = right;
        }
        if a[right] == y {
            last_y_pos = right;
        }

        while !cnt_map.is_empty() {
            if let Some((&num, _)) = cnt_map.last_key_value() {
                if num > x {
                    let cnt = cnt_map.get_mut(&a[left]).unwrap();
                    *cnt -= 1;
                    if *cnt == 0 {
                        cnt_map.remove(&a[left]);
                    }
                    left += 1;
                    continue;
                }
            }

            if let Some((&num, _)) = cnt_map.first_key_value() {
                if num < y {
                    let cnt = cnt_map.get_mut(&a[left]).unwrap();
                    *cnt -= 1;
                    if *cnt == 0 {
                        cnt_map.remove(&a[left]);
                    }
                    left += 1;
                    continue;
                }
            }

            break;
        }

        if last_x_pos == n || last_y_pos == n || last_x_pos.min(last_y_pos) < left {
            continue;
        }
        ans += last_x_pos.min(last_y_pos) - left + 1;
    }

    println!("{}", ans);
}
