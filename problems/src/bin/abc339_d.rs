use proconio::{input, marker::Chars};

const INF: usize = 1 << 30;

/// https://atcoder.jp/contests/abc339/tasks/abc339_d
/// https://atcoder.jp/contests/abc339/editorial/9211
/// 盤面の遷移をBFSで解く問題
/// N^4 頂点のグラフであって、頂点に 1 以上 N 以下の整数 4 つの組で番号が付けられたものを用意する
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut v_1 = (n, n);
    let mut v_2 = (n, n);
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'P' {
                if v_1 == (n, n) {
                    v_1 = (i, j);
                } else {
                    v_2 = (i, j);
                }
            }
        }
    }

    let dist = grid_bfs(&s, v_1, v_2);

    let mut ans = INF;
    for i in 0..n {
        for j in 0..n {
            ans = ans.min(dist[i][j][i][j]);
        }
    }

    println!("{}", if ans == INF { -1 } else { ans as i64 });
}

pub fn grid_bfs(
    field: &Vec<Vec<char>>,
    v_1: (usize, usize),
    v_2: (usize, usize),
) -> Vec<Vec<Vec<Vec<usize>>>> {
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    if field.is_empty() {
        return Vec::new();
    }
    let n = field.len();
    let mut dist = vec![vec![vec![vec![INF; n]; n]; n]; n];
    let mut que = std::collections::VecDeque::new();
    dist[v_1.0][v_1.1][v_2.0][v_2.1] = 0;
    que.push_back((v_1.0, v_1.1, v_2.0, v_2.1));
    while let Some((x_1, y_1, x_2, y_2)) = que.pop_front() {
        for dir in 0..4 {
            let nx_1 = (x_1 as i32 + dx[dir]).max(0).min(n as i32 - 1);
            let ny_1 = (y_1 as i32 + dy[dir]).max(0).min(n as i32 - 1);
            let nx_2 = (x_2 as i32 + dx[dir]).max(0).min(n as i32 - 1);
            let ny_2 = (y_2 as i32 + dy[dir]).max(0).min(n as i32 - 1);

            let mut nx_1 = nx_1 as usize;
            let mut ny_1 = ny_1 as usize;
            let mut nx_2 = nx_2 as usize;
            let mut ny_2 = ny_2 as usize;
            if field[nx_1][ny_1] == '#' {
                nx_1 = x_1;
                ny_1 = y_1;
            }
            if field[nx_2][ny_2] == '#' {
                nx_2 = x_2;
                ny_2 = y_2;
            }
            if dist[nx_1][ny_1][nx_2][ny_2] != INF {
                continue;
            }

            dist[nx_1][ny_1][nx_2][ny_2] = dist[x_1][y_1][x_2][y_2] + 1;
            que.push_back((nx_1, ny_1, nx_2, ny_2));
        }
    }
    dist
}
