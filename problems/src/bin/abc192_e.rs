use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
        abtk: [(Usize1,Usize1,usize,usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, t, k) in &abtk {
        graph[a].push((b, t, k));
        graph[b].push((a, t, k));
    }

    let dist = dijkstra(&graph, x);

    println!(
        "{}",
        if dist[y] == std::usize::MAX {
            -1
        } else {
            dist[y] as i64
        }
    );
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    time: usize,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub fn dijkstra(graph: &[Vec<(usize, usize, usize)>], start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![std::usize::MAX; n];
    let mut pq = std::collections::BinaryHeap::new();

    dist[start] = 0;
    for &(next_vertex, t, _) in &graph[start] {
        let new_time = t;
        if new_time < dist[next_vertex] {
            dist[next_vertex] = new_time;
            pq.push(Node {
                vertex: next_vertex,
                time: new_time,
            });
        }
    }

    while let Some(Node { vertex, time }) = pq.pop() {
        if dist[vertex] < time {
            continue;
        }
        for &(next_vertex, t, k) in &graph[vertex] {
            let new_time = t + (time + k - 1) / k * k;
            if new_time < dist[next_vertex] {
                dist[next_vertex] = new_time;
                pq.push(Node {
                    vertex: next_vertex,
                    time: new_time,
                });
            }
        }
    }
    dist
}
