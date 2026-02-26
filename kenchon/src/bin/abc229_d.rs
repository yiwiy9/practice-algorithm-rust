use proconio::{input, marker::Chars};

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
        s: Chars,
        k: usize,
    }

    let mut ans: usize = 0;
    let mut left = 0;
    let mut cur = 0;
    for right in 0..s.len() {
        if s[right] == '.' {
            cur += 1;
        }
        while cur > k {
            if s[left] == '.' {
                cur -= 1;
            }
            left += 1;
        }
        ans = ans.max(right - left + 1);
    }

    println!("{}", ans);
}
