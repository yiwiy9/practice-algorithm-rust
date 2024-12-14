use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h],
    }

    let mut starts = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'H' {
                starts.push((i, j));
            }
        }
    }

    starts.sort_by(|a, b| {
        ((a.0 as i64 - (h / 2) as i64).abs()).cmp(&(b.0 as i64 - (h / 2) as i64).abs())
    });

    let mut dist = HashMap::new();
    let mut humid = HashSet::new();
    for &start in &starts {
        grid_bfs(&s, d, &mut dist, &mut humid, start);
    }

    println!("{}", humid.len());
}

pub fn grid_bfs(
    field: &Vec<Vec<char>>,
    d: usize,
    dist: &mut HashMap<(usize, usize), usize>,
    humid: &mut HashSet<(usize, usize)>,
    s: (usize, usize),
) {
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    if field.is_empty() {
        return;
    }
    let h = field.len();
    let w = field[0].len();
    let mut que = std::collections::VecDeque::new();
    dist.insert(s, 0);
    que.push_back(s);
    humid.insert(s);
    while let Some((x, y)) = que.pop_front() {
        if dist[&(x, y)] >= d {
            continue;
        }
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
            if field[nx][ny] == 'H' {
                continue;
            }
            if dist.contains_key(&(nx, ny)) && dist[&(nx, ny)] <= dist[&(x, y)] + 1 {
                continue;
            }
            dist.insert((nx, ny), dist[&(x, y)] + 1);
            que.push_back((nx, ny));
            humid.insert((nx, ny));
        }
    }
}
