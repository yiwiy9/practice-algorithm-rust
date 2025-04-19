use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut s: Chars,
    }

    let mut map = HashMap::new();
    let mut que = std::collections::VecDeque::new();
    map.insert(s.clone(), 0);
    que.push_back(s);
    while let Some(t) = que.pop_front() {
        if t == vec!['a', 't', 'c', 'o', 'd', 'e', 'r'] {
            println!("{}", map[&t]);
            return;
        }

        for i in 1..t.len() {
            let mut u = t.clone();
            u.swap(i - 1, i);
            if !map.contains_key(&u) {
                map.insert(u.clone(), map[&t] + 1);
                que.push_back(u);
            }
        }
    }
}

// 最短距離なので DFS は不適
fn _rec(
    seen: &mut HashSet<Vec<char>>,
    memo: &mut HashMap<Vec<char>, usize>,
    s: &mut Vec<char>,
    cnt: usize,
) -> usize {
    if memo.contains_key(s) {
        return memo[s];
    }

    if s == &vec!['a', 't', 'c', 'o', 'd', 'e', 'r'] {
        return cnt;
    }

    let mut ans = usize::MAX;
    for i in 1..s.len() {
        let mut t = s.clone();
        t.swap(i - 1, i);
        if !seen.contains(&t) {
            seen.insert(t.clone());
            ans = ans.min(_rec(seen, memo, &mut t, cnt + 1));
            seen.remove(&t);
        }
    }

    memo.insert(s.clone(), ans);
    ans
}
