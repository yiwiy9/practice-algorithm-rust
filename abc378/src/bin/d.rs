use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut seen = HashSet::new();
            seen.insert((i, j));
            ans += dfs(&s, h, w, k, &mut seen, (i, j));
        }
    }

    println!("{}", ans);
}

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

pub fn dfs(
    field: &Vec<Vec<char>>,
    h: usize,
    w: usize,
    k: usize,
    seen: &mut HashSet<(usize, usize)>,
    v: (usize, usize),
) -> usize {
    if k == 0 {
        return 1;
    }

    let (x, y) = v;
    let mut res = 0;
    for dir in 0..4 {
        let nx = x as i32 + DX[dir];
        let ny = y as i32 + DY[dir];
        if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if field[nx][ny] == '#' {
            continue;
        }
        if seen.contains(&(nx, ny)) {
            continue;
        }
        seen.insert((nx, ny));
        res += dfs(field, h, w, k - 1, seen, (nx, ny));
        seen.remove(&(nx, ny));
    }

    res
}
