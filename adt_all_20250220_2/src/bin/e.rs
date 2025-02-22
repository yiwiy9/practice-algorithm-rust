use ac_library::Dsu;
use proconio::{input, marker::Chars};
use std::collections::HashSet;

const DX: [i32; 4] = [1, 1, 0, -1];
const DY: [i32; 4] = [0, 1, 1, 1];

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut dsu = Dsu::new(h * w);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            for k in 0..4 {
                let ni = i as i32 + DX[k];
                let nj = j as i32 + DY[k];
                if ni < 0 || nj < 0 || ni >= h as i32 || nj >= w as i32 {
                    continue;
                }

                let ni = ni as usize;
                let nj = nj as usize;
                if s[ni][nj] == '.' {
                    continue;
                }

                if dsu.same(i * w + j, ni * w + nj) {
                    continue;
                }

                dsu.merge(i * w + j, ni * w + nj);
            }
        }
    }

    let mut leaders = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            leaders.insert(dsu.leader(i * w + j));
        }
    }

    println!("{}", leaders.len());
}
