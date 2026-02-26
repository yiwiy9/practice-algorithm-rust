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

    let sum: usize = a.iter().sum();
    if sum % 10 != 0 {
        println!("No");
        return;
    }

    let target = sum / 10;

    let mut left = 0;
    let mut cur = 0;
    for right in 0..2 * n {
        cur += a[right % n];
        while cur > target {
            cur -= a[left % n];
            left += 1;
        }
        if cur == target {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
