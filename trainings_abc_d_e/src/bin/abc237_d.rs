use itertools::Itertools as _;
use proconio::{input, marker::Chars};
use std::collections::HashMap;
const INF: usize = 1 << 60;

/**
 * https://atcoder.jp/contests/abc237/tasks/abc237_d
 * https://atcoder.jp/contests/abc237/editorial/3319
 * 操作を逆順で考えたらもっと簡単にできるよ
 * => 両端キューの端に追加していくだけで実現できる
 */
fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    // 連結リストをHashMapで簡易的に実現する
    let mut prev_map = HashMap::new();
    let mut next_map = HashMap::new();
    prev_map.insert(0, INF);
    prev_map.insert(INF, 0);
    next_map.insert(0, INF);
    next_map.insert(INF, 0);

    for (i, &c) in s.iter().enumerate() {
        if c == 'R' {
            let next = *next_map.get(&i).unwrap();
            prev_map.insert(i + 1, i);
            prev_map.insert(next, i + 1);

            next_map.insert(i, i + 1);
            next_map.insert(i + 1, next);
        } else {
            let prev = *prev_map.get(&i).unwrap();
            prev_map.insert(i + 1, prev);
            prev_map.insert(i, i + 1);

            next_map.insert(prev, i + 1);
            next_map.insert(i + 1, i);
        }
    }

    let mut start = *prev_map.iter().find(|&(_, &prev)| prev == INF).unwrap().0;
    let mut ans = vec![];
    while start != INF {
        ans.push(start);
        start = *next_map.get(&start).unwrap();
    }

    println!("{}", ans.iter().join(" "));
}
