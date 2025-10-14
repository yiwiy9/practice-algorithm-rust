use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut s = (0, 0);
    let mut g = (0, 0);
    for (i, row) in a.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' {
                s = (i, j);
            } else if c == 'G' {
                g = (i, j);
            }
        }
    }

    let dist = grid_bfs(&a, s);

    let ans = dist[g.0][g.1][0].min(dist[g.0][g.1][1]) as i32;

    println!("{}", if ans != 1 << 30 { ans } else { -1 });
}

pub fn grid_bfs(field: &Vec<Vec<char>>, s: (usize, usize)) -> Vec<Vec<Vec<usize>>> {
    let inf: usize = 1 << 30;
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    if field.is_empty() {
        return Vec::new();
    }
    let h = field.len();
    let w = field[0].len();
    let mut dist = vec![vec![vec![inf; 2]; w]; h];
    let mut que = std::collections::VecDeque::new();
    dist[s.0][s.1][0] = 0;
    que.push_back((s.0, s.1, 0, 0));
    while let Some((x, y, z, dist_before)) = que.pop_front() {
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
            if field[nx][ny] == 'o' && z == 1 {
                continue;
            }
            if field[nx][ny] == 'x' && z == 0 {
                continue;
            }
            if dist[nx][ny][z] != inf {
                continue;
            }

            dist[nx][ny][z] = dist_before + 1;
            que.push_back((
                nx,
                ny,
                if field[nx][ny] == '?' { z ^ 1 } else { z },
                dist[nx][ny][z],
            ))
        }
    }
    dist
}
