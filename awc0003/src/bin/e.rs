use proconio::input;

/// https://atcoder.jp/contests/awc0003/tasks/awc0003_e
/// https://atcoder.jp/contests/awc0003/editorial/15889
/// 想定解法: bit DP（subset DP）
///
/// 問題の骨子（この実装の見方）
/// - 荷物は N 個（N<=15）なので、集合を bitmask (0..2^N-1) で管理できる
/// - dp[mask] = true  ⇔ これまでに処理したトラックだけで「mask の荷物を積み終えられる」
/// - 各トラック容量 C_j について、未使用の荷物集合 r から「このトラックに載せる subset v」を選び、
///   dp[mask | v] を true にする（v は r の部分集合、かつ sum(v) <= C_j）
///
/// 計算量感
/// - w_sum の前計算: O(N * 2^N)
/// - DP: 各状態 mask から未使用集合 r の submask を列挙するので、1トラックあたり最悪 O(3^N)
///   （∑_{mask} 2^{popcount(r)} = 3^N）
fn main() {
    input! {
        n: usize,
        m: usize,
        w: [usize; n],   // 各荷物の重さ
        c: [usize; m],   // 各トラックの容量（順に処理する）
    }

    // bitmask の最大値（全荷物集合）
    let n_bit_max = 1 << n;

    // w_sum[mask] = mask に含まれる荷物の重さ合計
    // これがあると「subset v をこのトラックに載せてよいか？」を O(1) で判定できる
    let mut w_sum = vec![0; n_bit_max];
    for bit in 0..n_bit_max {
        for i in 0..n {
            if bit & (1 << i) != 0 {
                w_sum[bit] += w[i];
            }
        }
    }

    // dp[mask] = true なら、これまでのトラックで mask の荷物を積める
    let mut dp = vec![false; n_bit_max];
    dp[0] = true; // 何も積まない状態は常に可能

    // トラックを1台ずつ処理
    for &c_j in &c {
        // 次のトラックまで使ったときに到達可能な状態
        // ※「このトラックを何回も使う」誤りを避けるため、dp を更新先 next_dp に分けている
        let mut next_dp = vec![false; n_bit_max];

        // すでに到達可能な積み済み状態 bit から遷移を作る
        for bit in 0..n_bit_max {
            if !dp[bit] {
                continue;
            }

            // このトラックに何も載せない選択肢（= 状態維持）
            next_dp[bit] = true;

            // not_used = まだ積んでいない荷物の集合 r
            // 全集合 (n_bit_max-1) から bit を引いたもの
            let not_used = (n_bit_max - 1) ^ bit;

            // not_used の部分集合 next_used (= v) を全列挙して「このトラックに載せる集合」を試す
            //
            // submask 列挙の定番:
            // v = r; v = (v-1) & r; ...; 0
            // → ちょうど 2^{popcount(r)} 個の submask を重複なく列挙できる
            //
            // ※このループは v>0 のみ回している（v=0 は状態維持で既に処理済み）
            let mut next_used = not_used;
            while next_used > 0 {
                // このトラック容量に収まるなら、その荷物集合を追加で積める
                if w_sum[next_used] <= c_j {
                    next_dp[bit | next_used] = true;
                }
                // 次の submask へ
                next_used = (next_used - 1) & not_used;
            }
        }

        // 次へ
        dp = next_dp;
    }

    // 全荷物 (mask = 2^N - 1) を積めるか
    println!("{}", if dp[n_bit_max - 1] { "Yes" } else { "No" })
}

// use proconio::input;
// use std::collections::HashSet;
// 本番ACの嘘解法
// fn main() {
//     input! {
//         n: usize,
//         m: usize,
//         mut w: [usize; n],
//         c: [usize; m],
//     }

//     let mut cur_c = c.clone();

//     w.sort_by(|a, b| b.cmp(a));
//     cur_c.sort_by(|a, b| b.cmp(a));

//     println!(
//         "{}",
//         if rec(n, m, &w, 0, &mut cur_c, &mut HashSet::new()) {
//             "Yes"
//         } else {
//             "No"
//         }
//     );
// }

// fn rec(
//     n: usize,
//     m: usize,
//     w: &Vec<usize>,
//     i: usize,
//     cur_c: &mut Vec<usize>,
//     memo: &mut HashSet<(usize, Vec<usize>)>,
// ) -> bool {
//     if i == n {
//         return true;
//     }

//     if memo.contains(&(i, cur_c.to_vec())) {
//         return false;
//     }

//     for j in 0..m {
//         if cur_c[j] < w[i] {
//             continue;
//         }

//         cur_c[j] -= w[i];
//         if rec(n, m, w, i + 1, cur_c, memo) {
//             return true;
//         }
//         cur_c[j] += w[i];
//     }

//     memo.insert((i, cur_c.clone()));

//     false
// }
