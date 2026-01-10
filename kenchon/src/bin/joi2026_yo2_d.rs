use proconio::input;
use superslice::*;

/// https://atcoder.jp/contests/joi2026yo2/tasks/joi2026_yo2_d
/// https://www2.ioi-jp.org/joi/2025/2026-yo2/2026-yo2-t4-review.pdf
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u64; n],
        cd: [(u64, u64); q],
    }

    a.sort();

    let mut pref = vec![0; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + a[i];
    }
    let total = pref[n];

    for (c, d) in cd {
        // smin = N - floor(C/D)
        // 直感：1枚追加の得 ≈ (残ってる個数)*D、コスト=C
        //      境目は (残ってる個数) ≈ C/D
        let t = c / d;
        let smin_i64 = n as i64 - t as i64;

        // smin <= 0 なら最初から増やす価値が薄い → k=0 が最適
        if smin_i64 <= 0 {
            println!("{}", total);
            continue;
        }

        // base = A_{smin}（1-indexedのつもりなので -1）
        let base = a[(smin_i64 as usize) - 1];

        // x は D の倍数しか選べない。
        // 凸な関数の谷（連続最小）の “左右に最も近い格子点” はこの2つだけ。
        let x1 = (base / d) * d;
        let x2 = base.div_ceil(d) * d;

        let ans = cost(&a, &pref, total, c, d, x1).min(cost(&a, &pref, total, c, d, x2));
        println!("{}", ans);
    }
}

/// x = D*k を固定したときの支払額を計算する
///
/// f(x) = Σ max(0, Ai - x) + C*(x/D)
///
/// 躓きポイント：
/// - 「Ai <= x になった個数」が増えるたびに、傾きが1ずつ変わる → 折れ線（凸）
/// - よって最小は “傾きが負→正に変わる付近” にしか来ない
fn cost(a: &[u64], pref: &[u64], total: u64, c: u64, d: u64, x: u64) -> u64 {
    // s = Ai <= x の個数（= 0円になった個数）
    let s = a.upper_bound(&x);

    // tail は Ai > x の部分
    let sum_tail = total - pref[s]; // Σ_{Ai > x} Ai
    let cnt_tail = (a.len() - s) as u64; // 個数

    // Σ_{Ai > x}(Ai - x) = Σ Ai - cnt*x
    let remaining = sum_tail - cnt_tail * x;

    let k = x / d; // x = D*k の関係
    remaining + c * k
}
