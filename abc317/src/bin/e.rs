use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut field = vec![vec!['.'; w]; h];
    let mut start = (0, 0);
    let mut goal = (0, 0);

    for i in 0..h {
        for j in 0..w {
            match a[i][j] {
                '.' => continue,
                '#' => field[i][j] = '#',
                'S' => start = (i, j),
                'G' => goal = (i, j),
                '^' => {
                    field[i][j] = '#';
                    let mut up_i = i as i32 - 1;
                    loop {
                        if up_i < 0 || a[up_i as usize][j] != '.' {
                            break;
                        }
                        field[up_i as usize][j] = '#';
                        up_i -= 1;
                    }
                }
                '<' => {
                    field[i][j] = '#';
                    let mut left_j = j as i32 - 1;
                    loop {
                        if left_j < 0 || a[i][left_j as usize] != '.' {
                            break;
                        }
                        field[i][left_j as usize] = '#';
                        left_j -= 1;
                    }
                }
                'v' => {
                    field[i][j] = '#';
                    let mut down_i = i + 1;
                    loop {
                        if down_i >= h || a[down_i][j] != '.' {
                            break;
                        }
                        field[down_i][j] = '#';
                        down_i += 1;
                    }
                }
                '>' => {
                    field[i][j] = '#';
                    let mut right_j = j + 1;
                    loop {
                        if right_j >= w || a[i][right_j] != '.' {
                            break;
                        }
                        field[i][right_j] = '#';
                        right_j += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let dist = grid_bfs(&field, start);

    println!(
        "{}",
        if dist[goal.0][goal.1] == 1 << 30 {
            -1
        } else {
            dist[goal.0][goal.1] as i32
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
