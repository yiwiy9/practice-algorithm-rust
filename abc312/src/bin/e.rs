use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

/**
 * https://atcoder.jp/contests/abc312/tasks/abc312_e
 * https://atcoder.jp/contests/abc312/editorial/6838
 * https://twitter.com/kyopro_friends/status/1685285988090904576?s=20
 *
 * 直接、直方体を考えるのではなく、1x1x1のマスがどの直方体に入ってるかを記録して、
 * 全マス（100x100x100）を全探索して、隣り合うマスが所属している直方体があればセットにぶち込む
 */
fn main() {
    input! {
        n: usize,
        xyz: [((usize,usize,usize),(usize,usize,usize)); n],
    }

    let mut grids = vec![vec![vec![0; 101]; 101]; 101];

    for (i, &((x1, y1, z1), (x2, y2, z2))) in xyz.iter().enumerate() {
        for x in x1..x2 {
            for y in y1..y2 {
                for z in z1..z2 {
                    grids[x][y][z] = i + 1;
                }
            }
        }
    }

    let mut ans = vec![BTreeSet::new(); n + 1];
    for x in 0..100 {
        for y in 0..100 {
            for z in 0..100 {
                if grids[x][y][z] == 0 {
                    continue;
                }
                if grids[x + 1][y][z] != 0 && grids[x + 1][y][z] != grids[x][y][z] {
                    ans[grids[x][y][z]].insert(grids[x + 1][y][z]);
                    ans[grids[x + 1][y][z]].insert(grids[x][y][z]);
                }
                if grids[x][y + 1][z] != 0 && grids[x][y + 1][z] != grids[x][y][z] {
                    ans[grids[x][y][z]].insert(grids[x][y + 1][z]);
                    ans[grids[x][y + 1][z]].insert(grids[x][y][z]);
                }
                if grids[x][y][z + 1] != 0 && grids[x][y][z + 1] != grids[x][y][z] {
                    ans[grids[x][y][z]].insert(grids[x][y][z + 1]);
                    ans[grids[x][y][z + 1]].insert(grids[x][y][z]);
                }
            }
        }
    }

    println!("{}", ans.iter().skip(1).map(|set| set.len()).join("\n"));
}
