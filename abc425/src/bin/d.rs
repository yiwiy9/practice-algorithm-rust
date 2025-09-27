use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let res = grid_bfs(&s);

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if res[i][j].0 == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

pub fn grid_bfs(field: &Vec<Vec<char>>) -> Vec<Vec<(i32, i32)>> {
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    if field.is_empty() {
        return Vec::new();
    }
    let h = field.len();
    let w = field[0].len();
    let mut color = vec![vec![(-1, -1); w]; h];
    let mut que = std::collections::VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            if field[i][j] == '#' {
                color[i][j] = (0, 0);
                que.push_back((i, j));
            }
        }
    }

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
            if color[nx][ny] != (-1, -1) {
                continue;
            }

            let cur_dist = color[x][y].1;
            let mut cnt = 0;
            for dir in 0..4 {
                let nnx = nx as i32 + dx[dir];
                let nny = ny as i32 + dy[dir];
                if nnx < 0 || h as i32 <= nnx || nny < 0 || w as i32 <= nny {
                    continue;
                }
                let nnx = nnx as usize;
                let nny = nny as usize;
                if color[nnx][nny].0 == 0 && color[nnx][nny].1 <= cur_dist {
                    cnt += 1;
                }
            }

            if cnt != 1 {
                color[nx][ny] = (1, cur_dist + 1);
                continue;
            }

            color[nx][ny] = (0, cur_dist + 1);
            que.push_back((nx, ny))
        }
    }
    color
}
