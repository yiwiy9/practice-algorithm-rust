use proconio::input;
use std::collections::VecDeque;

/// https://atcoder.jp/contests/joi2012yo/tasks/joi2012yo_e
/// https://drken1215.hatenablog.com/entry/2020/12/03/233500
/// => マップの外周に白マスを番兵として配置することで、建物の外側を意味する一つの連結成分ができる
fn main() {
    input! {
        w: usize,
        h: usize,
        a: [[usize; w]; h],
    }

    // 番兵(0)付きにする
    let hh = h + 2;
    let ww = w + 2;
    let mut g = vec![vec![0; ww]; hh];
    for x in 0..h {
        for y in 0..w {
            g[x + 1][y + 1] = a[x][y];
        }
    }

    // x(行)の偶奇で dy が変わる
    let dx: [[i32; 6]; 2] = [[1, 0, -1, 0, 1, -1], [1, 0, -1, 0, 1, -1]];
    let dy: [[i32; 6]; 2] = [[0, 1, 0, -1, -1, -1], [0, 1, 0, -1, 1, 1]];

    let mut seen = vec![vec![false; ww]; hh];
    let mut q = VecDeque::new();
    seen[0][0] = true;
    q.push_back((0, 0));

    let mut ans: usize = 0;

    while let Some((x, y)) = q.pop_front() {
        let p = x % 2; // 行の偶奇
        for dir in 0..6 {
            let nx = x as i32 + dx[p][dir];
            let ny = y as i32 + dy[p][dir];
            if nx < 0 || nx >= hh as i32 || ny < 0 || ny >= ww as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;

            if g[nx][ny] == 1 {
                // 外側から到達できる空きマスに接する建物の辺だけ数える
                ans += 1;
            } else if !seen[nx][ny] {
                seen[nx][ny] = true;
                q.push_back((nx, ny));
            }
        }
    }

    println!("{}", ans);
}
