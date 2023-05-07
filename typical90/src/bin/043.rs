use proconio::{input, marker::Chars, marker::Usize1};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:(Usize1,Usize1),
        t:(Usize1,Usize1),
        field:[Chars;h]
    }

    let dist = zero_one_bfs(&field, h, w, s);

    println!("{}", dist[t.0][t.1].iter().min().unwrap());
}

pub fn zero_one_bfs(
    field: &[Vec<char>],
    h: usize,
    w: usize,
    start: (usize, usize),
) -> Vec<Vec<Vec<usize>>> {
    let mut dist = vec![vec![vec![std::usize::MAX; 4]; w]; h];
    let mut deque = std::collections::VecDeque::new();
    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];

    let (s_x, s_y) = start;
    for dir in 0..4 {
        dist[start.0][start.1][dir] = 0;
        deque.push_front((s_x, s_y, dir));
    }

    while let Some((x, y, cur_dir)) = deque.pop_front() {
        for dir in 0..4 {
            let nx = x as i32 + dx[dir];
            let ny = y as i32 + dy[dir];
            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if field[nx][ny] == '#' {
                continue;
            }

            if dir == cur_dir {
                if dist[nx][ny][dir] > dist[x][y][cur_dir] {
                    dist[nx][ny][dir] = dist[x][y][cur_dir];
                    deque.push_front((nx, ny, dir));
                }
            } else {
                if dist[nx][ny][dir] > dist[x][y][cur_dir] + 1 {
                    dist[nx][ny][dir] = dist[x][y][cur_dir] + 1;
                    deque.push_back((nx, ny, dir));
                }
            }
        }
    }
    dist
}

// use proconio::{input, marker::Chars, marker::Usize1};
// use std::cmp::Ordering;
// use std::collections::BinaryHeap;

// type Vertex = (usize, usize, usize);

// #[derive(Debug, Clone, Eq, PartialEq)]
// struct Node {
//     vertex: Vertex,
//     cost: i64,
// }

// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.cmp(&self.cost)
//     }
// }

// impl PartialOrd for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// fn main() {
//     input! {
//         h:usize,
//         w:usize,
//         r_s:Usize1,
//         c_s:Usize1,
//         r_t:Usize1,
//         c_t:Usize1,
//         field:[Chars;h]
//     }

//     let dx = [1, 0, -1, 0];
//     let dy = [0, 1, 0, -1];

//     // マス（i,j)で上下左右の各方向を向く状態までの方向転換の最小回数
//     let mut dist = vec![vec![vec![std::i64::MAX; 4]; w]; h];
//     let mut pq = BinaryHeap::new();

//     for dir in 0..4 {
//         dist[r_s][c_s][dir] = 0;
//         pq.push(Node {
//             vertex: (r_s, c_s, dir),
//             cost: 0,
//         });
//     }

//     while let Some(Node { vertex, cost }) = pq.pop() {
//         let (x, y, cur_dir) = vertex;
//         if dist[x][y][cur_dir] < cost {
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

//             if field[nx][ny] == '#' {
//                 continue;
//             }

//             let new_cost = if dir == cur_dir { cost } else { cost + 1 };
//             if new_cost < dist[nx][ny][dir] {
//                 dist[nx][ny][dir] = new_cost;
//                 pq.push(Node {
//                     vertex: (nx, ny, dir),
//                     cost: new_cost,
//                 });
//             }
//         }
//     }

//     println!("{}", dist[r_t][c_t].iter().min().unwrap());
// }
