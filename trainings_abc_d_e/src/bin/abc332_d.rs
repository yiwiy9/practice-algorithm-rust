use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

const INF: usize = 1 << 60;

/**
 * https://atcoder.jp/contests/abc332/tasks/abc332_d
 * https://atcoder.jp/contests/abc332/editorial/7922
 *
 * 最短距離はBFS!!!!
 * => seen を戻ってくるたびに初期化してるのでいけると思ったが、
 *    memo は最初に見つけた答えを保持しているので、結局は最短距離ではなくなる
 */
fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[usize; w]; h],
        b: [[usize; w]; h],
    }

    // let res: usize = rec(h, w, &b, &mut a, &mut HashSet::new(), &mut HashMap::new());
    let dist = bfs(h, w, &a);

    println!(
        "{}",
        if let Some(&d) = dist.get(&b) {
            d as i64
        } else {
            -1
        }
    );
}

fn bfs(h: usize, w: usize, init_a: &Vec<Vec<usize>>) -> HashMap<Vec<Vec<usize>>, usize> {
    let mut dist = HashMap::new();
    let mut que = VecDeque::new();
    dist.insert(init_a.clone(), 0);
    que.push_back(init_a.clone());
    while let Some(mut a) = que.pop_front() {
        let cur_dist = dist[&a];

        for i in 0..h - 1 {
            change_rows(&mut a, i, i + 1);
            if !dist.contains_key(&a) {
                dist.insert(a.clone(), cur_dist + 1);
                que.push_back(a.clone());
            }
            change_rows(&mut a, i, i + 1);
        }

        for j in 0..w - 1 {
            change_cols(&mut a, j, j + 1);
            if !dist.contains_key(&a) {
                dist.insert(a.clone(), cur_dist + 1);
                que.push_back(a.clone());
            }
            change_cols(&mut a, j, j + 1);
        }
    }
    dist
}

fn rec(
    h: usize,
    w: usize,
    b: &[Vec<usize>],
    a: &mut Vec<Vec<usize>>,
    seen: &mut HashSet<Vec<Vec<usize>>>,
    memo: &mut HashMap<Vec<Vec<usize>>, usize>,
) -> usize {
    if check(h, w, a, b) {
        return 0;
    }

    if let Some(&v) = memo.get(a) {
        return v;
    }

    let mut res: usize = INF;
    for i in 0..h - 1 {
        change_rows(a, i, i + 1);
        if seen.contains(a) {
            change_rows(a, i, i + 1);
            continue;
        }
        seen.insert(a.clone());
        res = res.min(rec(h, w, b, a, seen, memo) + 1);
        seen.remove(a);
        change_rows(a, i, i + 1);
    }

    for j in 0..w - 1 {
        change_cols(a, j, j + 1);
        if seen.contains(a) {
            change_cols(a, j, j + 1);
            continue;
        }
        seen.insert(a.clone());
        res = res.min(rec(h, w, b, a, seen, memo) + 1);
        seen.remove(a);
        change_cols(a, j, j + 1);
    }

    memo.insert(a.clone(), res);
    res
}

fn check(h: usize, w: usize, a: &[Vec<usize>], b: &[Vec<usize>]) -> bool {
    for i in 0..h {
        for j in 0..w {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    true
}

fn change_rows(a: &mut Vec<Vec<usize>>, i: usize, j: usize) {
    a.swap(i, j);
}

fn change_cols(a: &mut Vec<Vec<usize>>, i: usize, j: usize) {
    for k in 0..a.len() {
        a[k].swap(i, j);
    }
}
