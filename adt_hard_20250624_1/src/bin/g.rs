use proconio::input;
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }

    let mut reached = HashSet::new();
    let mut que = VecDeque::new();
    reached.insert(a.clone());
    que.push_back((a, 0));

    while let Some((current, steps)) = que.pop_front() {
        if current == b {
            println!("{}", steps);
            return;
        }

        for i in 0..h - 1 {
            let mut next = current.clone();
            next.swap(i, i + 1);
            if !reached.contains(&next) {
                reached.insert(next.clone());
                que.push_back((next, steps + 1));
            }
        }
        for j in 0..w - 1 {
            let mut next = current.clone();
            for i in 0..h {
                next[i].swap(j, j + 1);
            }
            if !reached.contains(&next) {
                reached.insert(next.clone());
                que.push_back((next, steps + 1));
            }
        }
    }

    println!("-1");
}
