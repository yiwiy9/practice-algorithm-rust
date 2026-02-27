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
    let mut cnt_vec = vec![0; 100_010];
    for right in 0..n {
        cnt_vec[a[right]] += 1;
        while cnt_vec[a[right]] > 1 {
            cnt_vec[a[left]] -= 1;
            left += 1;
        }
        ans = ans.max(right - left + 1);
    }

    println!("{}", ans);
}
