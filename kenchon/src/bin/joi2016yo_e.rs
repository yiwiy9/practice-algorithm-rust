use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: usize,
        p: usize,
        q: usize,
        c: [Usize1; k],
        ab: [(Usize1,Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let dangerous_dist = bfs(&graph, &c);
    let mut dangerous = vec![0; n];
    for i in 0..n {
        if dangerous_dist[i] == 0 {
            dangerous[i] = 2;
        } else if dangerous_dist[i] <= s {
            dangerous[i] = 1;
        }
    }

    let dist = dijkstra(&graph, &dangerous, p, q);

    println!("{}", dist[n - 1]);
}

pub fn bfs(graph: &[Vec<usize>], c: &[usize]) -> Vec<usize> {
    let inf = (1 << 30) as usize;
    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut que = std::collections::VecDeque::new();

    for &c_i in c {
        dist[c_i] = 0;
        que.push_back(c_i);
    }

    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if dist[v] != inf {
                continue;
            }
            dist[v] = dist[u] + 1;
            que.push_back(v);
        }
    }
    dist
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
pub fn dijkstra(graph: &[Vec<usize>], dangerous: &[usize], p: usize, q: usize) -> Vec<usize> {
    let inf: usize = 1 << 60;
    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    dist[0] = 0;
    pq.push(Node { vertex: 0, cost: 0 });
    while let Some(Node { vertex, cost }) = pq.pop() {
        if dist[vertex] < cost {
            continue;
        }

        for &next_vertex in &graph[vertex] {
            if dangerous[next_vertex] == 2 {
                continue;
            }

            let new_cost = cost
                + if next_vertex == n - 1 {
                    0
                } else if dangerous[next_vertex] == 0 {
                    p
                } else {
                    q
                };
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
