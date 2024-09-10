use itertools::Itertools as _;
use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(Usize1,Usize1,usize); m],
    }

    let mut graph = vec![vec![]; n];
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        graph[a].push((b, c, i + 1));
        graph[b].push((a, c, i + 1));
    }

    // 最小全域木ではないので要注意
    let ans = dijkstra(&graph, 0);

    println!("{}", ans.iter().join(" "));
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    cost: usize,
    index: usize,
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
pub fn dijkstra(graph: &[Vec<(usize, usize, usize)>], start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![INF; n];
    let mut pq = std::collections::BinaryHeap::new();
    let mut ans = vec![];
    dist[start] = 0;
    pq.push(Node {
        vertex: start,
        cost: 0,
        index: INF,
    });
    while let Some(Node {
        vertex,
        cost,
        index,
    }) = pq.pop()
    {
        if dist[vertex] < cost {
            continue;
        }
        if index != INF {
            ans.push(index);
        }
        for &(next_vertex, edge_cost, edge_index) in &graph[vertex] {
            let new_cost = cost + edge_cost;
            if new_cost < dist[next_vertex] {
                dist[next_vertex] = new_cost;
                pq.push(Node {
                    vertex: next_vertex,
                    cost: new_cost,
                    index: edge_index,
                });
            }
        }
    }
    ans
}
