use proconio::{input, marker::Usize1};

/**
 * E - I Hate Sigma Problems
 * https://atcoder.jp/contests/abc371/tasks/abc371_e
 * https://atcoder.jp/contests/abc371/editorial/10922
 *
 * => f(l,r)は「A[l]からA[r]に1が入ってたら+1」「2が入ってたら+1」…
 * => **値ごとに独立に考えられることに気づく**
 * => 「1が入ってる区間の個数」「2が入ってる区間の個数」…の和
 * => これらは余事象を使って計算できる
 */
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    // 計算の都合上、両端（0, n+1）を追加
    let mut b = vec![vec![0]; n];
    for i in 0..n {
        b[a[i]].push(i + 1); // a[i]がない区間を求めるために自分の後ろのインデックスを記録
    }
    for i in 0..n {
        b[i].push(n + 1);
    }

    // 1からnまでの値が含まれる区間の個数
    // 長さnの区間の数+...長さ2の+長さ1の= 1+2+3+...+n = n(n+1)/2
    // × n (種類数)
    let mut ans = n * (n + 1) / 2 * n;
    for i in 0..n {
        for j in 0..b[i].len() - 1 {
            let l = b[i][j];
            let r = b[i][j + 1] - 1; // b[a[i]にはi+1を格納してるため、右端は-1
            ans -= (r - l) * (r - l + 1) / 2;
        }
    }

    println!("{}", ans);
}

// use proconio::input;
// use std::collections::HashSet;

// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n],
//     }

//     let mut num_first_i = vec![vec![]; n + 1];
//     for (i, &a_i) in a.iter().enumerate().rev() {
//         num_first_i[a_i].push(i);
//     }

//     let mut set = HashSet::new();
//     let mut b = vec![0; n];
//     for (i, &a_i) in a.iter().enumerate() {
//         set.insert(a_i);
//         b[i] = set.len();
//     }

//     let mut b_sum = b.iter().sum::<usize>();

//     let mut ans = 0;
//     for i in 0..n {
//         ans += b_sum;

//         num_first_i[a[i]].pop();
//         if num_first_i[a[i]].is_empty() {
//             b_sum -= n - i
//         } else {
//             b_sum -= num_first_i[a[i]].last().unwrap() - i
//         }
//     }

//     println!("{}", ans);
// }
