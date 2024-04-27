use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        abx: [(usize,usize,Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    for (i, &(a, b, x)) in abx.iter().enumerate() {
        graph[i].push((i + 1, a));
        graph[i].push((x, b));
    }

    let dist = dijkstra(&graph, 0);

    println!("{}", dist[n - 1]);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
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
pub fn dijkstra(graph: &[Vec<(usize, usize)>], start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![std::usize::MAX; n];
    let mut pq = std::collections::BinaryHeap::new();
    dist[start] = 0;
    pq.push(Node {
        vertex: start,
        cost: 0,
    });
    while let Some(Node { vertex, cost }) = pq.pop() {
        if dist[vertex] < cost {
            continue;
        }
        for &(next_vertex, edge_cost) in &graph[vertex] {
            let new_cost = cost + edge_cost;
            if new_cost < dist[next_vertex] {
                dist[next_vertex] = new_cost;
                pq.push(Node {
                    vertex: next_vertex,
                    cost: new_cost,
                });
            }
        }
    }
    dist
}
