use ac_library::Dsu;
use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let convert_ij = |i: usize, j: usize| -> usize { i * w + j };

    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    let n = h * w;
    let mut dsu = Dsu::new(n);
    for (i, row) in a.iter().enumerate() {
        for (j, &a_ij) in row.iter().enumerate() {
            for dir in 0..4 {
                let ni = i as i32 + dx[dir];
                let nj = j as i32 + dy[dir];
                if ni < 0 || h as i32 <= ni || nj < 0 || w as i32 <= nj {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if a[ni][nj] == a_ij {
                    dsu.merge(convert_ij(i, j), convert_ij(ni, nj));
                }
            }
        }
    }

    let mut map: HashMap<(usize, usize), HashSet<usize>> = HashMap::new();
    for (i, row) in a.iter().enumerate() {
        for (j, &a_ij) in row.iter().enumerate() {
            let cur_leader = dsu.leader(convert_ij(i, j));
            for dir in 0..4 {
                let ni = i as i32 + dx[dir];
                let nj = j as i32 + dy[dir];
                if ni < 0 || h as i32 <= ni || nj < 0 || w as i32 <= nj {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                map.entry((cur_leader, a[ni][nj]))
                    .or_default()
                    .insert(cur_leader);
                if a[ni][nj] != a_ij {
                    map.entry((cur_leader, a[ni][nj]))
                        .or_default()
                        .insert(dsu.leader(convert_ij(ni, nj)));
                }
            }
        }
    }

    println!(
        "{}",
        map.values()
            .map(|leader_set| leader_set
                .iter()
                .map(|&leader| dsu.size(leader))
                .sum::<usize>())
            .max()
            .unwrap_or(1)
    );
}
