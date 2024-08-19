use proconio::{input, marker::Chars};
const INF: usize = 1 << 30;

fn main() {
    input! {
        h: usize,
        _: usize,
        c: [Chars; h],
    }

    let dist = grid_bfs(&c, (0, 0));

    println!(
        "{}",
        dist.iter()
            .map(|row| row.iter().filter(|&&v| v != INF).max().unwrap_or(&0))
            .max()
            .unwrap()
            + 1
    );
}

pub fn grid_bfs(field: &Vec<Vec<char>>, s: (usize, usize)) -> Vec<Vec<usize>> {
    let inf: usize = INF;
    let dx: [i32; 2] = [1, 0];
    let dy: [i32; 2] = [0, 1];
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
        for dir in 0..2 {
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
