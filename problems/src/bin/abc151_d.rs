use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _: usize,
        s: [Chars; h],
    }

    let mut ans = 0;
    for (i, s_row) in s.iter().enumerate() {
        for (j, &c) in s_row.iter().enumerate() {
            if c == '.' {
                let max_dist = grid_bfs(&s, (i, j))
                    .iter()
                    .map(|dist_row| {
                        *dist_row
                            .iter()
                            .filter(|&&dist| dist != 1 << 30)
                            .max()
                            .unwrap_or(&0)
                    })
                    .max()
                    .unwrap_or(0);

                ans = ans.max(max_dist);
            }
        }
    }

    println!("{}", ans);
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
