use itertools::Itertools;
use proconio::{input, marker::Usize1};

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    let mut inverse_graph = vec![vec![]; n];
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        inverse_graph[b].push((a, c));
    }

    let dist = dijkstra(n, m, &graph, &inverse_graph, 0);
    let target = dist[n - 1]
        .iter()
        .enumerate()
        .filter(|&(_, &dist_n)| dist_n <= l)
        .collect_vec();

    println!(
        "{}",
        if target.is_empty() {
            -1
        } else {
            target[0].0 as i64
        }
    );
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    times: usize,
    cost: usize,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .times
            .cmp(&self.times)
            .then(other.cost.cmp(&self.cost))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub fn dijkstra(
    n: usize,
    m: usize,
    graph: &[Vec<(usize, usize)>],
    inverse_graph: &[Vec<(usize, usize)>],
    start: usize,
) -> Vec<Vec<usize>> {
    let mut dist = vec![vec![INF; m + 1]; n];
    let mut pq = std::collections::BinaryHeap::new();
    dist[start][0] = 0;
    pq.push(Node {
        vertex: start,
        times: 0,
        cost: 0,
    });
    while let Some(Node {
        vertex,
        times,
        cost,
    }) = pq.pop()
    {
        if dist[vertex][times] < cost {
            continue;
        }
        for &(next_vertex, edge_cost) in &graph[vertex] {
            let new_cost = cost + edge_cost;
            if new_cost < dist[next_vertex][times] {
                dist[next_vertex][times] = new_cost;
                pq.push(Node {
                    vertex: next_vertex,
                    times,
                    cost: new_cost,
                });
            }
        }
        if times < m {
            for &(next_vertex, edge_cost) in &inverse_graph[vertex] {
                let new_cost = cost + edge_cost;
                let next_times = times + 1;
                if new_cost < dist[next_vertex][times] && new_cost < dist[next_vertex][next_times] {
                    dist[next_vertex][next_times] = new_cost;
                    pq.push(Node {
                        vertex: next_vertex,
                        times: next_times,
                        cost: new_cost,
                    });
                }
            }
        }
    }
    dist
}
