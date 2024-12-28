use proconio::input;

/// https://atcoder.jp/contests/abc385/tasks/abc385_c
/// https://atcoder.jp/contests/abc385/editorial/11655
///
/// 間隔のループは調和級数なので O(N log N) で解ける
/// => 1 + 1/2 + 1/3 + ... + 1/N = O(log N)
///
/// 3重ループでも O(N^2 log N) で解ける
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = 1;

    // スタート位置のループ
    for i in 0..n {
        // 間隔のループ
        for j in 1..n {
            let mut cur = 0;
            let mut idx = i;
            while idx < n {
                if h[idx] != h[i] {
                    break;
                }
                cur += 1;
                idx += j;
            }

            ans = ans.max(cur);
        }
    }

    println!("{}", ans);
}

// use proconio::input;
// use std::collections::BTreeSet;

// fn main() {
//     input! {
//         n: usize,
//         h: [usize; n],
//     }

//     let mut num_set = vec![BTreeSet::new(); 3001];
//     for i in 0..n {
//         num_set[h[i]].insert(i);
//     }

//     let mut ans = 0;
//     for i in 0..n {
//         let mut diff = vec![];
//         for &num in num_set[h[i]].range(i + 1..) {
//             diff.push(num - i);
//         }

//         let mut cur = 1;
//         for diff_i in &diff {
//             let mut cnt = 1;
//             loop {
//                 if !num_set[h[i]].contains(&(diff_i * cnt + i)) {
//                     break;
//                 }
//                 cnt += 1;
//             }
//             cur = cur.max(cnt);
//         }

//         ans = ans.max(cur);
//     }

//     println!("{}", ans);
// }
