// use proconio::input;
// use std::collections::{HashMap, HashSet};

// // Nim で解けた！！
// fn main() {
//     input! {
//         n: usize,
//         ab: [(usize,usize); n],
//     }

//     println!(
//         "{}",
//         if rec(&ab, &mut vec![false; n], &mut HashMap::new()) == 0 {
//             "Aoki"
//         } else {
//             "Takahashi"
//         }
//     );
// }

// fn rec(ab: &[(usize, usize)], used: &mut Vec<bool>, memo: &mut HashMap<Vec<bool>, usize>) -> usize {
//     if let Some(&g) = memo.get(used) {
//         return g;
//     }

//     let mut grundy = HashSet::new();

//     for i in 0..ab.len() {
//         if used[i] {
//             continue;
//         }
//         for j in i + 1..ab.len() {
//             if used[j] {
//                 continue;
//             }
//             if ab[i].0 == ab[j].0 || ab[i].1 == ab[j].1 {
//                 used[i] = true;
//                 used[j] = true;
//                 grundy.insert(rec(ab, used, memo));
//                 used[i] = false;
//                 used[j] = false;
//             }
//         }
//     }

//     let mut mex = 0;
//     while grundy.contains(&mex) {
//         mex += 1;
//     }

//     memo.insert(used.clone(), mex);
//     mex
// }

/**
 * https://atcoder.jp/contests/abc354/tasks/abc354_e
 * https://atcoder.jp/contests/abc354/editorial/10034
 * 想定解の bit DP に挑戦（再帰と遷移のイメージが近い）
 *
 * dp[b] := bのうち立っているビットのカードが場に残っている状態で先手が勝つことができる場合はtrue、そうでない場合はfalse
 */
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
    }

    let mut dp = vec![false; 1 << n];
    for bit in 1..(1 << n) {
        let mut can_win = false;

        for i in 0..n {
            if bit & (1 << i) == 0 {
                continue;
            }

            for j in i + 1..n {
                if bit & (1 << j) == 0 {
                    continue;
                }

                if (ab[i].0 == ab[j].0 || ab[i].1 == ab[j].1) && !dp[bit ^ (1 << i) ^ (1 << j)] {
                    can_win = true;
                }
            }
        }

        dp[bit] = can_win;
    }

    println!(
        "{}",
        if dp[(1 << n) - 1] {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}
