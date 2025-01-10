use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/adt_all_20250109_1/tasks/abc213_e
/// https://atcoder.jp/contests/adt_all_20250109_1/editorial/2397
///
/// 高橋君が下図の T にいるとき、パンチを 1 回繰り出すことで、
/// 区画の元の状態によらず * のいずれかに移動することができます。
/// .***.
/// *****
/// **T**
/// *****
/// .***.
/// したがって、徒歩による上下左右への移動をコスト 0 、
/// パンチを繰り出した直後の * のいずれかへの移動をコスト 1 として、
/// 01BFSによりこの問題を解くことができました。計算量は O(HW) です。
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let dist = zero_one_bfs(&s, h, w, (0, 0));

    println!("{}", dist[h - 1][w - 1]);
}

pub fn zero_one_bfs(
    field: &[Vec<char>],
    h: usize,
    w: usize,
    start: (usize, usize),
) -> Vec<Vec<usize>> {
    let mut dist = vec![vec![std::usize::MAX; w]; h];
    let mut deque = std::collections::VecDeque::new();

    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];
    let tdx = [
        -1, 0, 1, -2, -1, 0, 1, 2, -2, -1, 1, 2, -2, -1, 0, 1, 2, -1, 0, 1,
    ];
    let tdy = [
        -2, -2, -2, -1, -1, -1, -1, -1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2,
    ];

    dist[start.0][start.1] = 0;
    deque.push_front(start);
    while let Some((x, y)) = deque.pop_front() {
        for dir in 0..4 {
            let nx = x as i32 + dx[dir];
            let ny = y as i32 + dy[dir];
            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if field[nx][ny] != '#' && dist[nx][ny] > dist[x][y] {
                dist[nx][ny] = dist[x][y];
                deque.push_front((nx, ny));
            }
        }

        for tdir in 0..20 {
            let nx = x as i32 + tdx[tdir];
            let ny = y as i32 + tdy[tdir];
            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if dist[nx][ny] > dist[x][y] + 1 {
                dist[nx][ny] = dist[x][y] + 1;
                deque.push_back((nx, ny));
            }
        }
    }
    dist
}
