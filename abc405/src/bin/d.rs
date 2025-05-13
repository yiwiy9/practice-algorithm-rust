use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
    }

    let ans = grid_bfs(&s);

    println!(
        "{}",
        ans.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

pub fn grid_bfs(field: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let inf: usize = 1 << 30;
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    let d_ans: [char; 4] = ['^', '<', 'v', '>'];

    if field.is_empty() {
        return Vec::new();
    }
    let h = field.len();
    let w = field[0].len();

    let mut dist = vec![vec![inf; w]; h];
    let mut que = std::collections::VecDeque::new();
    let mut ans = vec![vec!['#'; w]; h];

    for i in 0..h {
        for j in 0..w {
            if field[i][j] == 'E' {
                dist[i][j] = 0;
                que.push_back((i, j));
                ans[i][j] = 'E';
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
            if dist[nx][ny] != inf {
                continue;
            }
            dist[nx][ny] = dist[x][y] + 1;
            que.push_back((nx, ny));
            ans[nx][ny] = d_ans[dir];
        }
    }
    ans
}
