use proconio::{input, marker::Chars};
use std::collections::{BTreeMap, VecDeque};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        mut t: Chars,
    }

    s.extend(['.', '.'].iter());
    t.extend(['.', '.'].iter());

    let dist_s = bfs(n, &s);

    println!(
        "{}",
        if dist_s.contains_key(&t) {
            dist_s[&t]
        } else {
            -1
        }
    );
}

pub fn bfs(n: usize, init_s: &Vec<char>) -> BTreeMap<Vec<char>, i64> {
    let mut dist = BTreeMap::new();
    let mut que = VecDeque::new();
    dist.insert(init_s.clone(), 0);
    que.push_back(init_s.clone());

    while let Some(s) = que.pop_front() {
        let empty_i = s.iter().position(|&c| c == '.').unwrap();

        for i in 0..n + 1 {
            if i + 1 == empty_i || i == empty_i || i == empty_i + 1 {
                continue;
            }

            let mut new_s = s.clone();
            new_s.swap(i, empty_i);
            new_s.swap(i + 1, empty_i + 1);

            if dist.contains_key(&new_s) {
                continue;
            }
            dist.insert(new_s.clone(), dist[&s] + 1);
            que.push_back(new_s);
        }
    }

    dist
}
