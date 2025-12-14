use proconio::{input, marker::Chars};
use std::collections::BTreeMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let dist = grid_bfs(&s, (0, 0));

    println!(
        "{}",
        if dist[h - 1][w - 1] == 1 << 30 {
            -1
        } else {
            dist[h - 1][w - 1] as i32
        }
    );
}

pub fn grid_bfs(field: &Vec<Vec<char>>, s: (usize, usize)) -> Vec<Vec<usize>> {
    let inf: usize = 1 << 30;
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    if field.is_empty() {
        return Vec::new();
    }
    let h = field.len();
    let w = field[0].len();

    let mut warps = BTreeMap::new();
    for (i, row) in field.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '.' || c == '#' {
                continue;
            }
            warps.entry(c).or_insert(vec![]).push((i, j));
        }
    }

    let mut dist = vec![vec![inf; w]; h];
    let mut que = std::collections::VecDeque::new();
    dist[s.0][s.1] = 0;
    que.push_back(s);
    while let Some((x, y)) = que.pop_front() {
        for dir in 0..4 {
            let nx = x as i32 + dx[dir];
            let ny = y as i32 + dy[dir];
            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if field[nx][ny] == '#' {
                continue;
            }
            if dist[nx][ny] != inf {
                continue;
            }
            dist[nx][ny] = dist[x][y] + 1;
            que.push_back((nx, ny))
        }

        if let Some(warp) = warps.get(&field[x][y]) {
            for &(nx, ny) in warp {
                if dist[nx][ny] != inf {
                    continue;
                }
                dist[nx][ny] = dist[x][y] + 1;
                que.push_back((nx, ny))
            }
            warps.remove(&field[x][y]);
        }
    }
    dist
}
