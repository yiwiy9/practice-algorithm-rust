// use proconio::{input, marker::Chars};

// fn main() {
//     input! {
//         s: Chars,
//     }

//     let mut pos = vec![vec![]; 26];
//     for (i, &c) in s.iter().enumerate() {
//         pos[(c as u8 - b'A') as usize].push(i);
//     }

//     let mut ans = 0;
//     for i in 0..26 {
//         if pos[i].len() < 2 {
//             continue;
//         }

//         let mut excess_sum = pos[i][0];
//         for j in 1..pos[i].len() {
//             // 先頭にある想定でカウント - それまでに前に出てきた文字位置の和（過剰に足してる分）
//             ans += (pos[i][j] - 1) * j - excess_sum;
//             excess_sum += pos[i][j];
//         }
//     }

//     println!("{}", ans);
// }

/**
 * 真ん中に注目した方がイメージしやすい
 * https://atcoder.jp/contests/abc375/tasks/abc375_d
 * https://atcoder.jp/contests/abc375/editorial/11139
 */
use proconio::{input, marker::Chars};
use superslice::*;
fn main() {
    input! {
        s: Chars,
    }

    let mut pos = vec![vec![]; 26];
    for (i, &c) in s.iter().enumerate() {
        pos[(c as u8 - b'A') as usize].push(i);
    }

    let mut ans = 0;
    for i in 0..s.len() {
        for c in 0..26 {
            let left_cnt = pos[c].lower_bound(&i);
            let right_cnt = pos[c].len() - pos[c].upper_bound(&i);
            ans += left_cnt * right_cnt;
        }
    }
    println!("{}", ans);
}
