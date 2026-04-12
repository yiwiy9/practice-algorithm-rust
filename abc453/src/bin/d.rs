use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::VecDeque;

const INF: usize = 1 << 60;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut start = (h, w);
    let mut goal = (h, w);
    for (i, row) in s.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' {
                start = (i, j);
            } else if c == 'G' {
                goal = (i, j);
            }
        }
    }

    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    let dir_c: [char; 4] = ['D', 'R', 'U', 'L'];

    let mut dist = vec![vec![vec![INF; 4]; w]; h];
    let mut que = VecDeque::new();
    for dir in 0..4 {
        dist[start.0][start.1][dir] = 0;
        que.push_back((start.0, start.1, dir));
    }

    while let Some((x, y, dir_in)) = que.pop_front() {
        for dir_out in 0..4 {
            if s[x][y] == 'o' && dir_in != dir_out {
                continue;
            }
            if s[x][y] == 'x' && dir_in == dir_out {
                continue;
            }

            let nx = x as i32 + dx[dir_out];
            let ny = y as i32 + dy[dir_out];
            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if s[nx][ny] == '#' {
                continue;
            }

            if dist[nx][ny][dir_out] != INF {
                continue;
            }

            dist[nx][ny][dir_out] = dist[x][y][dir_in] + 1;
            que.push_back((nx, ny, dir_out))
        }
    }

    let min_dist = *dist[goal.0][goal.1].iter().min().unwrap();
    if min_dist == INF {
        println!("No");
        return;
    }
    println!("Yes");

    let mut track = vec![];
    let mut cur = goal;
    let mut dir_out = 0;
    while cur != start {
        let (x, y) = cur;

        let mut min_dir_in = dir_out;
        let mut min_before = cur;
        let mut min_dist = INF;

        for dir_in in 0..4 {
            if s[x][y] == 'o' && dir_in != dir_out {
                continue;
            }
            if s[x][y] == 'x' && dir_in == dir_out {
                continue;
            }
            if dist[x][y][dir_in] == INF {
                continue;
            }

            let nx = x as i32 + dx[(dir_in + 2) % 4];
            let ny = y as i32 + dy[(dir_in + 2) % 4];
            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;

            if dist[x][y][dir_in] < min_dist {
                min_dir_in = dir_in;
                min_before = (nx, ny);
                min_dist = dist[x][y][dir_in];
            }
        }
        track.push(dir_c[min_dir_in]);
        cur = min_before;
        dir_out = min_dir_in;
    }
    track.reverse();

    println!("{}", track.iter().join(""));
}
