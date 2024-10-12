use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ldkcab: [(i64,i64,i64,i64,Usize1,Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(l, d, k, c, a, b) in &ldkcab {
        graph[b].push((a, l, d, k, c));
    }

    let dist = dijkstra(&graph, n - 1);

    for &time in dist.iter().take(n - 1) {
        if time == -1 {
            println!("Unreachable");
        } else {
            println!("{}", time);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    time: i64,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub fn dijkstra(graph: &[Vec<(usize, i64, i64, i64, i64)>], start: usize) -> Vec<i64> {
    let n = graph.len();
    let mut dist = vec![-1; n];
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(Node {
        vertex: start,
        time: std::i64::MAX,
    });
    while let Some(Node { vertex, time }) = pq.pop() {
        if dist[vertex] > time {
            continue;
        }
        for &(next_vertex, l, d, k, c) in &graph[vertex] {
            if time < l + c {
                continue;
            }

            let new_k = (k - 1).min((time - l - c) / d);
            let new_time = l + d * new_k;

            if new_time > dist[next_vertex] {
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
