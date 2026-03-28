use proconio::input;
use std::collections::BinaryHeap;

/// https://atcoder.jp/contests/joi2018yo/tasks/joi2018_yo_e
/// https://atcoder.jp/contests/joi2018yo/editorial/2658
///
/// 盤面全体をdistとpqに持ってMLE
/// => 盤面全体は不要で、本質的に必要なのは、今いる位置からスタートへの最短距離
/// => なぜなら進み方を変えるとそれは最短経路ではなくなるため、今きた道を次も必ず通る
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let ans = dijkstra(&a);

    println!("{}", ans);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    d: usize,
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
pub fn dijkstra(field: &[Vec<usize>]) -> usize {
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    let h = field.len();
    let w = field[0].len();

    let mut ans: usize = 1 << 60;
    let mut dist = vec![vec![vec![1 << 60; h * w]; w]; h];
    let mut pq = BinaryHeap::new();

    dist[0][0][0] = 0;
    pq.push(Node {
        x: 0,
        y: 0,
        d: 0,
        cost: 0,
    });

    while let Some(Node { x, y, d, cost }) = pq.pop() {
        if dist[x][y][d] < cost {
            continue;
        }

        let new_d = d + 1;
        if new_d >= h * w {
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

            let new_cost = cost + 1 + field[nx][ny] + (d * 2 * field[nx][ny]);

            if nx == h - 1 && ny == w - 1 {
                ans = ans.min(new_cost - d - 1);
                continue;
            }

            if new_cost < dist[nx][ny][new_d] {
                dist[nx][ny][new_d] = new_cost;
                pq.push(Node {
                    x: nx,
                    y: ny,
                    d: new_d,
                    cost: new_cost,
                });
            }
        }
    }

    ans
}
