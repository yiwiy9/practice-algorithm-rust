use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
    }

    println!("{}", grid_bfs(&s));
}

pub fn grid_bfs(field: &Vec<Vec<char>>) -> usize {
    let dx: [i32; 8] = [1, 1, 1, 0, -1, -1, -1, 0];
    let dy: [i32; 8] = [1, 0, -1, -1, -1, 0, 1, 1];
    if field.is_empty() {
        return 0;
    }
    let h = field.len();
    let w = field[0].len();
    let mut current = field
        .iter()
        .map(|row| {
            row.iter()
                .map(|&c| if c == '.' { 0 } else { c as i32 - '0' as i32 })
                .collect_vec()
        })
        .collect_vec();
    let mut dist = vec![vec![0; w]; h];
    let mut que = std::collections::VecDeque::new();

    for (x, row) in field.iter().enumerate() {
        for (y, &c) in row.iter().enumerate() {
            if c != '.' {
                continue;
            }

            for dir in 0..8 {
                let nx = x as i32 + dx[dir];
                let ny = y as i32 + dy[dir];
                if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if current[nx][ny] > 0 {
                    current[nx][ny] -= 1;
                    if current[nx][ny] == 0 {
                        dist[nx][ny] = 1;
                        que.push_back((nx, ny));
                    }
                }
            }
        }
    }

    while let Some((x, y)) = que.pop_front() {
        for dir in 0..8 {
            let nx = x as i32 + dx[dir];
            let ny = y as i32 + dy[dir];
            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if current[nx][ny] > 0 {
                current[nx][ny] -= 1;
                if current[nx][ny] == 0 {
                    dist[nx][ny] = dist[x][y] + 1;
                    que.push_back((nx, ny));
                }
            }
        }
    }

    *dist
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
}
