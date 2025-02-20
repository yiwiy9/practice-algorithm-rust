use proconio::{input, marker::Chars};
use std::collections::VecDeque;

/// https://atcoder.jp/contests/adt_all_20250219_3/tasks/abc329_e
/// https://atcoder.jp/contests/adt_all_20250219_3/editorial/7724
///
/// 操作を逆順に見ると、以下のような問題に変わります。
/// - S に対して以下の操作を繰り返し行うことで、S の文字を全て # にすることができるか判定せよ。
///     - 全ての j(0≤j<M) に対して「Si+j = Tj または Si+j = # 」を満たすような i(0≤i<N−M+1) を1つ選び、
///       Si,Si+1,…,Si+M−1 を全て # で置き換える。
///
/// この問題を解くためには、i を探すパートに毎回 O(N) かけていては間に合いません。
/// => 最初に良いiをキューに入れて、多始点BFS で解く
fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: Chars,
        t: Chars,
    }

    let check_i = |s: &Vec<char>, i: usize| -> bool {
        for j in 0..m {
            if s[i + j] != t[j] && s[i + j] != '#' {
                return false;
            }
        }
        true
    };

    let mut used = vec![false; n];
    let mut que = VecDeque::new();

    for i in 0..n - m + 1 {
        if check_i(&s, i) {
            used[i] = true;
            que.push_back(i);
        }
    }

    while let Some(i) = que.pop_front() {
        for j in 0..m {
            s[i + j] = '#';
        }

        let left = if i > m - 1 { i - m + 1 } else { 0 };
        let right = (i + m - 1).min(n - m);
        for i in left..=right {
            if !used[i] && check_i(&s, i) {
                used[i] = true;
                que.push_back(i);
            }
        }
    }

    println!(
        "{}",
        if s.iter().all(|&c| c == '#') {
            "Yes"
        } else {
            "No"
        }
    );
}
