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
                start.0 = i;
                start.1 = j;
            } else if s[i][j] == 'G' {
                goal.0 = i;
                goal.1 = j;
            }
        }
    }

    let dist = grid_bfs(&s, start);

    let ans = dist[goal.0][goal.1][0].min(dist[goal.0][goal.1][1]);

    println!("{}", if ans == 1 << 30 { -1 } else { ans as i64 });
}

pub fn grid_bfs(field: &Vec<Vec<char>>, s: (usize, usize)) -> Vec<Vec<Vec<usize>>> {
    let inf: usize = 1 << 30;
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    let h = field.len();
    let w = field[0].len();
    let mut dist = vec![vec![vec![inf; 2]; w]; h];
    let mut que = std::collections::VecDeque::new();

    dist[s.0][s.1][0] = 0;
    dist[s.0][s.1][1] = 0;
    que.push_back((s, true));
    que.push_back((s, false));

    while let Some(((x, y), can_vertical)) = que.pop_front() {
        for dir in 0..4 {
            if can_vertical {
                if dir == 1 || dir == 3 {
                    continue;
                }
            } else {
                if dir == 0 || dir == 2 {
                    continue;
                }
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
            if dist[nx][ny][if can_vertical { 0 } else { 1 }] != inf {
                continue;
            }
            dist[nx][ny][if can_vertical { 0 } else { 1 }] =
                dist[x][y][if can_vertical { 1 } else { 0 }] + 1;
            que.push_back(((nx, ny), !can_vertical));
        }
    }
    dist
}
