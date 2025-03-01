use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push((v, 0));
        graph[v].push((u, 1));
    }

    let dist = dijkstra(&graph, x);

    println!("{}", dist[n - 1][0].min(dist[n - 1][1]));
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    is_reverse: usize,
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
pub fn dijkstra(graph: &[Vec<(usize, usize)>], x: usize) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut dist = vec![vec![std::usize::MAX; 2]; n];
    let mut pq = std::collections::BinaryHeap::new();
    dist[0][0] = 0;
    pq.push(Node {
        vertex: 0,
        is_reverse: 0,
        cost: 0,
    });
    while let Some(Node {
        vertex,
        is_reverse,
        cost,
    }) = pq.pop()
    {
        if dist[vertex][is_reverse] < cost {
            continue;
        }
        for &(next_vertex, reverse_edge) in &graph[vertex] {
            let new_cost = if is_reverse == reverse_edge {
                cost + 1
            } else {
                cost + x + 1
            };
            let new_is_reverse = if is_reverse == reverse_edge {
                is_reverse
            } else {
                (is_reverse + 1) % 2
            };

            if new_cost < dist[next_vertex][new_is_reverse] {
                dist[next_vertex][new_is_reverse] = new_cost;
                pq.push(Node {
                    vertex: next_vertex,
                    is_reverse: new_is_reverse,
                    cost: new_cost,
                });
            }
        }
    }
    dist
}
