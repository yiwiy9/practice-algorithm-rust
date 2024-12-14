use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    let mut seen = HashMap::new();
    for i in 0..n {
        seen.insert(xy[i], false);
    }

    let mut ans = 0;
    for i in 0..n {
        if seen[&xy[i]] {
            continue;
        }
        let (x, y) = xy[i];
        dfs(&mut seen, x, y);
        ans += 1;
    }

    println!("{}", ans);
}

const DX: [i32; 6] = [-1, -1, 0, 0, 1, 1];
const DY: [i32; 6] = [-1, 0, -1, 1, 0, 1];

pub fn dfs(seen: &mut HashMap<(i32, i32), bool>, x: i32, y: i32) {
    seen.insert((x, y), true);
    for dir in 0..6 {
        let next_x = x + DX[dir];
        let next_y = y + DY[dir];
        if !seen.contains_key(&(next_x, next_y)) || seen[&(next_x, next_y)] {
            continue;
        }
        dfs(seen, next_x, next_y);
    }
}
