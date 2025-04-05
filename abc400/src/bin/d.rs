use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
        a: Usize1,
        b: Usize1,
        c: Usize1,
        d: Usize1,
    }

    let dist = dijkstra(&s, (a, b));

    println!("{}", dist[c][d]);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    cost: usize,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub fn dijkstra(field: &Vec<Vec<char>>, s: (usize, usize)) -> Vec<Vec<usize>> {
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    let h = field.len();
    let w = field[0].len();

    let mut dist = vec![vec![1 << 30; w]; h];
    let mut pq = std::collections::BinaryHeap::new();

    dist[s.0][s.1] = 0;
    pq.push(Node {
        x: s.0,
        y: s.1,
        cost: 0,
    });

    while let Some(Node { x, y, cost }) = pq.pop() {
        if dist[x][y] < cost {
            continue;
        }

        for dir in 0..4 {
            let nx = x as i32 + dx[dir];
            let ny = y as i32 + dy[dir];
            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;

            if field[nx][ny] == '.' {
                if cost < dist[nx][ny] {
                    dist[nx][ny] = cost;
                    pq.push(Node { x: nx, y: ny, cost });
                }
            } else {
                if cost + 1 < dist[nx][ny] {
                    dist[nx][ny] = cost + 1;
                    pq.push(Node {
                        x: nx,
                        y: ny,
                        cost: cost + 1,
                    });
                }
            }

            let nnx = x as i32 + dx[dir] * 2;
            let nny = y as i32 + dy[dir] * 2;
            if nnx < 0 || h as i32 <= nnx || nny < 0 || w as i32 <= nny {
                continue;
            }
            let nnx = nnx as usize;
            let nny = nny as usize;

            if field[nnx][nny] == '.' && field[nx][ny] == '.' {
                if cost < dist[nnx][nny] {
                    dist[nnx][nny] = cost;
                    pq.push(Node {
                        x: nnx,
                        y: nny,
                        cost,
                    });
                }
            } else {
                if cost + 1 < dist[nnx][nny] {
                    dist[nnx][nny] = cost + 1;
                    pq.push(Node {
                        x: nnx,
                        y: nny,
                        cost: cost + 1,
                    });
                }
            }
        }
    }
    dist
}
