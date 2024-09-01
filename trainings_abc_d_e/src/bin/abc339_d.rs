use proconio::{input, marker::Chars};
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut players = vec![];
    for (i, row) in s.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'P' {
                players.push((i, j));
            }
        }
    }
    players.sort();

    println!("{}", grid_bfs(n as i32, &s, players[0], players[1]));
}

pub fn grid_bfs(
    n: i32,
    field: &[Vec<char>],
    player_s: (usize, usize),
    player_l: (usize, usize),
) -> i64 {
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    let mut dist = HashMap::new();
    let mut que = VecDeque::new();
    dist.insert((player_s, player_l), 0);
    que.push_back((player_s, player_l));
    while let Some(((sx, sy), (lx, ly))) = que.pop_front() {
        let d = dist[&((sx, sy), (lx, ly))];

        for dir in 0..4 {
            let mut n_sx = (sx as i32 + dx[dir]).min(n - 1).max(0) as usize;
            let mut n_sy = (sy as i32 + dy[dir]).min(n - 1).max(0) as usize;
            let mut n_lx = (lx as i32 + dx[dir]).min(n - 1).max(0) as usize;
            let mut n_ly = (ly as i32 + dy[dir]).min(n - 1).max(0) as usize;

            if field[n_sx][n_sy] == '#' {
                n_sx = sx;
                n_sy = sy;
            }
            if field[n_lx][n_ly] == '#' {
                n_lx = lx;
                n_ly = ly;
            }

            let mut n_s = (n_sx, n_sy);
            let mut n_l = (n_lx, n_ly);
            if n_s == n_l {
                return d + 1;
            }
            if n_s > n_l {
                std::mem::swap(&mut n_s, &mut n_l);
            }

            if dist.contains_key(&(n_s, n_l)) {
                continue;
            }

            dist.insert((n_s, n_l), d + 1);
            que.push_back((n_s, n_l))
        }
    }
    -1
}
