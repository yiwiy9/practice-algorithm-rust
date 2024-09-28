use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        n: usize,
        rce: [(Usize1,Usize1,usize); n],
    }

    let mut start = (h, w);
    let mut goal = (h, w);
    for (i, row) in a.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' {
                start = (i, j);
            }
            if c == 'T' {
                goal = (i, j);
            }
        }
    }

    let mut rce_queue = VecDeque::new();
    let mut rce_map = BTreeMap::new();
    for &(r, c, e) in &rce {
        if (r, c) == start {
            rce_queue.push_back((r, c, e));
        } else {
            rce_map.insert((r, c), e);
        }
    }

    while let Some(start) = rce_queue.pop_front() {
        let dist = grid_bfs(&a, (start.0, start.1));
        for (i, row) in dist.iter().enumerate() {
            for (j, &dist_ij) in row.iter().enumerate() {
                if dist_ij <= start.2 {
                    if (i, j) == goal {
                        println!("Yes");
                        return;
                    }

                    if let Some(&e) = rce_map.get(&(i, j)) {
                        rce_queue.push_back((i, j, e));
                        rce_map.remove(&(i, j));
                    }
                }
            }
        }
    }

    println!("No");
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
    }
    dist
}
