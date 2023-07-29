use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut leaps = vec![];
    for i in 0..=((m as f64).sqrt() as usize) {
        for j in 0..=((m as f64).sqrt() as usize) {
            if i * i + j * j == m {
                leaps.push((i, j));
            }
        }
    }

    let dist = bfs(n, &leaps);

    println!("{}", dist.iter().map(|row| row.iter().join(" ")).join("\n"));
}

const DX: [i32; 4] = [1, 1, -1, -1];
const DY: [i32; 4] = [1, -1, 1, -1];

pub fn bfs(n: usize, leaps: &Vec<(usize, usize)>) -> Vec<Vec<i32>> {
    let mut dist = vec![vec![-1; n]; n];
    let mut que = std::collections::VecDeque::new();
    dist[0][0] = 0;
    que.push_back((0, 0));
    while let Some((x, y)) = que.pop_front() {
        for &(leap_x, leap_y) in leaps {
            for dir in 0..4 {
                let nx = x as i32 + (leap_x as i32 * DX[dir]);
                let ny = y as i32 + (leap_y as i32 * DY[dir]);

                if nx < 0 || n as i32 <= nx || ny < 0 || n as i32 <= ny {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;
                if dist[nx][ny] != -1 {
                    continue;
                }
                dist[nx][ny] = dist[x][y] + 1;
                que.push_back((nx, ny));
            }
        }
    }
    dist
}
