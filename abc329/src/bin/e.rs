use proconio::{input, marker::Chars};
use std::collections::VecDeque;

/**
 * https://atcoder.jp/contests/abc329/tasks/abc329_e
 * https://atcoder.jp/contests/abc329/editorial/7724
 * 操作を逆順に見る
 * => S の文字を全て # にすることができるか判定せよ
 * => 貪欲法で解ける
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: Chars,
        t: Chars,
    }

    let mut seen: Vec<bool> = vec![false; n - m + 1];
    let mut que = VecDeque::new();

    let mut check = |i: usize, que: &mut VecDeque<usize>, s: &Vec<char>| {
        if seen[i] {
            return;
        }
        let ok = (0..m).all(|j| s[i + j] == '#' || s[i + j] == t[j]);
        if ok {
            seen[i] = true;
            que.push_back(i);
        }
    };

    for i in 0..(n - m + 1) {
        check(i, &mut que, &s);
    }

    while let Some(i) = que.pop_front() {
        for j in 0..m {
            s[i + j] = '#';
        }

        let min_j = if i > m { i - m + 1 } else { 0 };
        let max_j = (i + m - 1).min(n - m);
        for j in min_j..=max_j {
            check(j, &mut que, &s);
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
