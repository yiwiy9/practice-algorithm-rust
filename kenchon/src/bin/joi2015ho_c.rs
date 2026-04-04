use proconio::{input, marker::Usize1};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    input! {
        n: usize,
        m: usize,
        c: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    let mut d_sum = 0;
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
        d_sum += c;
    }

    let dist = dijkstra(&graph, 0);

    let mut min_heap =
        BinaryHeap::from_iter(dist.into_iter().enumerate().map(|(i, d)| Reverse((d, i))));
    let (mut x, start_i) = min_heap.pop().unwrap().0;
    let mut underground_set = HashSet::new();
    underground_set.insert(start_i);
    let mut ans: usize = c * x + d_sum;

    while !min_heap.is_empty() {
        x = min_heap.peek().unwrap().0 .0;
        while let Some(Reverse((d, i))) = min_heap.pop() {
            if d > x {
                min_heap.push(Reverse((d, i)));
                break;
            }

            for &(adj_v, adj_d) in &graph[i] {
                if underground_set.contains(&adj_v) {
                    d_sum -= adj_d;
                }
            }

            underground_set.insert(i);
        }
        ans = ans.min(c * x + d_sum);
    }

    println!("{}", ans);
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
    let inf: usize = 1 << 60;
    let n = graph.len();
    let mut dist = vec![inf; n];
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
