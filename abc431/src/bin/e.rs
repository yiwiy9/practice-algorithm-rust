use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/abc431/tasks/abc431_e
/// https://atcoder.jp/contests/abc431/editorial/14483
/// => いい加減「頂点倍加」を手札に加える
///
/// 1. 1マスの各辺に着目して、その辺を通過する2つの向きそれぞれに対応する頂点を作る
/// 2. 1マス内のそれぞれの頂点に辺を張る
/// 3. タイプを変更しないと到達できない場合はコスト:1、そうでなければコスト:0
/// 4. 01-BFS で解ける
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            s: [Chars; h],
        }

        println!("{}", zero_one_bfs(&s, h, w));
    }
}

pub fn zero_one_bfs(field: &[Vec<char>], h: usize, w: usize) -> usize {
    let inf: usize = 1 << 60;
    let mut dist = vec![vec![vec![inf; 4]; w]; h];
    let mut deque = std::collections::VecDeque::new();
    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];
    deque.push_front((0, -1, 1, 0));
    while let Some((pre_x, pre_y, dir, cost)) = deque.pop_front() {
        let x = pre_x + dx[dir];
        let y = pre_y + dy[dir];
        if x < 0 || h as i32 <= x || y < 0 || w as i32 <= y {
            continue;
        }
        let x = x as usize;
        let y = y as usize;

        for next_dir in 0..4 {
            let mut cost_zero = false;

            if dir ^ next_dir == 2 {
                continue;
            } else if dir ^ next_dir == 0 {
                cost_zero = field[x][y] == 'A'
            } else if dir ^ next_dir == 1 {
                cost_zero = field[x][y] == 'B'
            } else {
                cost_zero = field[x][y] == 'C'
            }

            if cost_zero {
                if dist[x][y][next_dir] > cost {
                    dist[x][y][next_dir] = cost;
                    deque.push_front((x as i32, y as i32, next_dir, cost));
                }
            } else {
                if dist[x][y][next_dir] > cost + 1 {
                    dist[x][y][next_dir] = cost + 1;
                    deque.push_back((x as i32, y as i32, next_dir, cost + 1));
                }
            }
        }
    }
    dist[h - 1][w - 1][1]
}

// -------- 本番後のupsolve ----------

// use proconio::{input, marker::Chars};

// const INF: usize = 1 << 60;

// fn main() {
//     input! {
//         t: usize,
//     }

//     for _ in 0..t {
//         input! {
//             h: usize,
//             _w: usize,
//             s: [Chars; h],
//         }

//         let dist = dijkstra(&s);
//         let mut ans = INF;
//         for dir in 0..4 {
//             if dir == 3 {
//                 if s[0][0] == 'A' {
//                     ans = ans.min(dist[0][0][dir]);
//                 } else {
//                     ans = ans.min(dist[0][0][dir] + 1);
//                 }
//             } else if dir == 2 {
//                 if s[0][0] == 'B' {
//                     ans = ans.min(dist[0][0][dir]);
//                 } else {
//                     ans = ans.min(dist[0][0][dir] + 1);
//                 }
//             }
//         }
//         println!("{}", ans);
//     }
// }

// #[derive(Debug, Clone, Eq, PartialEq)]
// struct Node {
//     x: usize,
//     y: usize,
//     d: usize,
//     cost: usize,
// }
// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         other.cost.cmp(&self.cost)
//     }
// }
// impl PartialOrd for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }
// pub fn dijkstra(field: &[Vec<char>]) -> Vec<Vec<Vec<usize>>> {
//     let dx: [i32; 4] = [1, 0, -1, 0];
//     let dy: [i32; 4] = [0, 1, 0, -1];

//     let h = field.len();
//     let w = field[0].len();

//     let mut dist = vec![vec![vec![INF; 4]; w]; h];
//     let mut pq = std::collections::BinaryHeap::new();

//     dist[h - 1][w - 1][3] = 0;
//     pq.push(Node {
//         x: h - 1,
//         y: w - 1,
//         d: 3,
//         cost: 0,
//     });

//     while let Some(Node { x, y, d, cost }) = pq.pop() {
//         if dist[x][y][d] < cost {
//             continue;
//         }

//         for dir in 0..4 {
//             let nx = x as i32 + dx[dir];
//             let ny = y as i32 + dy[dir];
//             if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
//                 continue;
//             }
//             let nx = nx as usize;
//             let ny = ny as usize;

//             let mut new_cost = cost;
//             if d == dir {
//                 if field[x][y] != 'A' {
//                     new_cost += 1;
//                 }
//             } else if (d + 2) % 4 == dir {
//                 continue;
//             } else if (d + dir) % 4 == 1 {
//                 if field[x][y] != 'B' {
//                     new_cost += 1;
//                 }
//             } else {
//                 if field[x][y] != 'C' {
//                     new_cost += 1;
//                 }
//             }

//             if new_cost < dist[nx][ny][dir] {
//                 dist[nx][ny][dir] = new_cost;
//                 pq.push(Node {
//                     x: nx,
//                     y: ny,
//                     d: dir,
//                     cost: new_cost,
//                 });
//             }
//         }
//     }
//     dist
// }
