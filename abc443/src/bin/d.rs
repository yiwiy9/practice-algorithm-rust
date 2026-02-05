// use proconio::{input, marker::Usize1};
// use std::collections::BTreeMap;

// fn main() {
//     input! {
//         t: usize,
//     }

//     for _ in 0..t {
//         input! {
//             n: usize,
//             r: [Usize1; n],
//         }

//         let mut num_vec = vec![vec![]; n];
//         for (i, &r_i) in r.iter().enumerate() {
//             num_vec[r_i].push(i);
//         }

//         let mut map: BTreeMap<usize, usize> = BTreeMap::new();
//         for (num, ids) in num_vec.iter().enumerate() {
//             for &i in ids {
//                 let mut final_num = num;
//                 if let Some((j, before_num)) = map.range(..i).next_back() {
//                     final_num = final_num.min(before_num + (i - j));
//                 }
//                 if let Some((j, next_num)) = map.range(i..).next() {
//                     final_num = final_num.min(next_num + (j - i));
//                 }
//                 map.insert(i, final_num);
//             }
//         }

//         println!(
//             "{}",
//             (0..n).map(|i| r[i] - map.get(&i).unwrap()).sum::<usize>()
//         )
//     }
// }

use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc443/tasks/abc443_d
/// https://atcoder.jp/contests/abc443/editorial/15179
/// まず、i=2,3,…,N について順に、以下を行う。
///     Ri = min(Ri, Ri−1 + 1) に更新する。
/// 次に、i=N−1,N−2,…,1 について順に、以下を行う。
///     Ri = min(Ri, Ri+1 + 1) に更新する。
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            r: [Usize1; n],
        }

        let mut final_r = r.clone();
        for i in 1..n {
            final_r[i] = final_r[i].min(final_r[i - 1] + 1);
        }
        for i in (0..n - 1).rev() {
            final_r[i] = final_r[i].min(final_r[i + 1] + 1);
        }

        println!("{}", (0..n).map(|i| r[i] - final_r[i]).sum::<usize>())
    }
}
