use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut not_visited = HashSet::new();
    for &xy_i in &xy {
        not_visited.insert(xy_i);
    }

    let mut ans = 0;
    for i in 0..n {
        if !not_visited.contains(&xy[i]) {
            continue;
        }

        dfs(&mut not_visited, xy[i]);

        ans += 1;
    }

    println!("{}", ans);
}

const DIRS: [(i64, i64); 6] = [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)];

fn dfs(not_visited: &mut HashSet<(i64, i64)>, v: (i64, i64)) {
    not_visited.remove(&v);

    for &(dx, dy) in &DIRS {
        let next = (v.0 + dx, v.1 + dy);
        if not_visited.contains(&next) {
            dfs(not_visited, next);
        }
    }
}
