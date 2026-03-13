use itertools::Itertools;
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
        m: usize,
        mut ab: [(usize,usize); n],
    }

    ab.sort();

    let mut ans = vec![0; m + 1];
    let mut a_left_i = 0;
    let mut b_left = m;
    for right in (ab[n - 1].0)..=m {
        while a_left_i < n {
            if ab[a_left_i].1 > right {
                break;
            }
            b_left = b_left.min(ab[a_left_i].1);
            a_left_i += 1;
        }

        let left = if a_left_i < n {
            ab[a_left_i].0.min(b_left)
        } else {
            b_left
        };

        let k_min = right - left + 1;
        let k_max = right;
        ans[k_min - 1] += 1;
        ans[k_max] -= 1;
    }

    for i in 0..m {
        ans[i + 1] += ans[i];
    }

    println!("{}", ans.iter().take(m).join(" "));
}
