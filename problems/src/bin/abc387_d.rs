use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i, j);
            }
            if s[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }

    println!("{}", grid_bfs(&s, start, goal));
}

pub fn grid_bfs(field: &Vec<Vec<char>>, s: (usize, usize), g: (usize, usize)) -> isize {
    let inf: usize = 1 << 30;
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    let h = field.len();
    let w = field[0].len();
    let mut dist_vertical = vec![vec![inf; w]; h];
    let mut dist_horizontal = vec![vec![inf; w]; h];
    let mut que = std::collections::VecDeque::new();
    dist_vertical[s.0][s.1] = 0;
    dist_horizontal[s.0][s.1] = 0;
    que.push_back((s.0, s.1, 0));
    que.push_back((s.0, s.1, 1));
    while let Some((x, y, from_horizon)) = que.pop_front() {
        for dir in 0..4 {
            if from_horizon == 0 && dir % 2 == 1 {
                continue;
            }
            if from_horizon == 1 && dir % 2 == 0 {
                continue;
            }

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

            if from_horizon == 0 {
                if dist_vertical[nx][ny] != inf {
                    continue;
                }
                dist_vertical[nx][ny] = dist_horizontal[x][y] + 1;
                que.push_back((nx, ny, 1));
            } else {
                if dist_horizontal[nx][ny] != inf {
                    continue;
                }
                dist_horizontal[nx][ny] = dist_vertical[x][y] + 1;
                que.push_back((nx, ny, 0));
            }
        }
    }

    if dist_vertical[g.0][g.1] == inf && dist_horizontal[g.0][g.1] == inf {
        return -1;
    } else {
        return dist_vertical[g.0][g.1].min(dist_horizontal[g.0][g.1]) as isize;
    }
}
