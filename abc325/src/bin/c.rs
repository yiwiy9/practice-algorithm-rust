use ac_library::Dsu;
use proconio::{input, marker::Chars};
use std::collections::BTreeSet;

const DX: [i32; 4] = [1, 1, 1, 0];
const DY: [i32; 4] = [-1, 0, 1, 1];

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let calc_idx = |i: usize, j: usize| i * w + j;

    let mut dsu = Dsu::new(h * w);
    for (i, s_row) in s.iter().enumerate() {
        for (j, &c) in s_row.iter().enumerate() {
            if c == '.' {
                continue;
            }

            for dir in 0..4 {
                let mut nx = i as i32 + DX[dir];
                let mut ny = j as i32 + DY[dir];
                if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if s[nx][ny] == '#' {
                    dsu.merge(calc_idx(i, j), calc_idx(nx, ny));
                }
            }
        }
    }

    let mut root_set = BTreeSet::new();
    for (i, s_row) in s.iter().enumerate() {
        for (j, &c) in s_row.iter().enumerate() {
            if c == '#' {
                root_set.insert(dsu.leader(calc_idx(i, j)));
            }
        }
    }
    println!("{}", root_set.len());
}
