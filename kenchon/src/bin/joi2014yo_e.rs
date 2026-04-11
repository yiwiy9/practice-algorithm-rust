use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/joi2014yo/tasks/joi2014yo_e
/// - 固定するもの: 状態を (街 i, 残り j 本進める) に拡張する
/// - 欲しい量: dist[i][j] = その状態にいる最小コスト
/// - 必要クエリ: 乗り換え遷移と道路移動遷移
/// - 単調性 / 境界: 辺重み非負なので Dijkstra
///
/// AC: 19分
fn main() {
    input! {
        n: usize,
        k: usize,
        cr: [(usize,usize); n],
        ab: [(Usize1,Usize1); k],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    println!("{}", dijkstra(&graph, &cr));
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    cost: usize,
    remain: usize,
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
pub fn dijkstra(graph: &[Vec<usize>], cr: &[(usize, usize)]) -> usize {
    let inf: usize = 1 << 60;
    let n = graph.len();
    let mut dist = vec![vec![inf; n + 1]; n];
    let mut pq = std::collections::BinaryHeap::new();
    dist[0][0] = 0;
    pq.push(Node {
        vertex: 0,
        cost: 0,
        remain: 0,
    });
    while let Some(Node {
        vertex,
        cost,
        remain,
    }) = pq.pop()
    {
        if dist[vertex][remain] < cost {
            continue;
        }

        let (cur_c, cur_r) = cr[vertex];
        let new_cost = cost + cur_c;
        if new_cost < dist[vertex][cur_r] {
            dist[vertex][cur_r] = new_cost;
            pq.push(Node {
                vertex,
                cost: new_cost,
                remain: cur_r,
            });
        }

        if remain == 0 {
            continue;
        }

        let new_remain = remain - 1;
        for &next_vertex in &graph[vertex] {
            if cost < dist[next_vertex][new_remain] {
                dist[next_vertex][new_remain] = cost;
                pq.push(Node {
                    vertex: next_vertex,
                    cost,
                    remain: new_remain,
                });
            }
        }
    }
    *dist[n - 1].iter().min().unwrap()
}
