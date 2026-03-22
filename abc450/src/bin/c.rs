use std::collections::HashSet;

use ac_library::Dsu;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let calc = |x: usize, y: usize| -> usize { x * w + y };

    let outside = h * w;
    let mut dsu = Dsu::new(outside + 1);

    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    for (x, row) in s.iter().enumerate() {
        for (y, &s_ij) in row.iter().enumerate() {
            if s_ij == '#' {
                continue;
            }

            let cur = calc(x, y);

            for dir in 0..4 {
                let nx = x as i32 + dx[dir];
                let ny = y as i32 + dy[dir];
                if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                    dsu.merge(cur, outside);
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if s[nx][ny] == '#' {
                    continue;
                }
                dsu.merge(cur, calc(nx, ny));
            }
        }
    }

    let outside_leader = dsu.leader(outside);
    let mut leader_set = HashSet::new();
    for (x, row) in s.iter().enumerate() {
        for (y, &s_ij) in row.iter().enumerate() {
            if s_ij == '#' {
                continue;
            }

            let cur = calc(x, y);
            let leader = dsu.leader(cur);
            if leader != outside_leader {
                leader_set.insert(leader);
            }
        }
    }

    println!("{}", leader_set.len());
}
