use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
        k: usize,
        a: [Usize1; k],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut living = vec![false; n];
    for &a_i in &a {
        living[a_i] = true;
    }

    let dist = dijkstra(&graph, &living, 0);

    let ans = dist[n - 1].iter().min().copied().unwrap();

    println!(
        "{}",
        if ans == std::usize::MAX {
            -1
        } else {
            ans as i64
        }
    );
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    cost: (usize, usize),
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
pub fn dijkstra(graph: &[Vec<usize>], living: &[bool], start: usize) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut dist = vec![vec![std::usize::MAX; 5]; n];
    let mut pq = std::collections::BinaryHeap::new();

    if living[start] {
        dist[start][1] = 0;
        pq.push(Node {
            vertex: start,
            cost: (0, 1),
        });
    } else {
        dist[start][0] = 0;
        pq.push(Node {
            vertex: start,
            cost: (0, 0),
        });
    }
    while let Some(Node { vertex, cost }) = pq.pop() {
        if dist[vertex][cost.1] < cost.0 {
            continue;
        }
        for &next_vertex in &graph[vertex] {
            let new_dist = cost.0 + 1;
            let new_cnt = if living[next_vertex] { cost.1 + 1 } else { 0 };
            if new_cnt >= 5 {
                continue;
            }
            if new_dist < dist[next_vertex][new_cnt] {
                dist[next_vertex][new_cnt] = new_dist;
                pq.push(Node {
                    vertex: next_vertex,
                    cost: (new_dist, new_cnt),
                });
            }
        }
    }
    dist
}
