use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
        p: [isize; n],
    }

    let mut start_vec = vec![];
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        if p[i] == -1 {
            start_vec.push((i, a[i]));
        } else {
            graph[p[i] as usize - 1].push((i, a[i]));
        }
    }

    let dist = dijkstra(&graph, &start_vec);

    println!(
        "{}",
        dist.iter()
            .rev()
            .position(|&d| d <= x)
            .map(|d| (n - d) as isize)
            .unwrap_or(-1)
    );
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
pub fn dijkstra(graph: &[Vec<(usize, usize)>], start_vec: &Vec<(usize, usize)>) -> Vec<usize> {
    let inf: usize = 1 << 60;
    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();

    for &(v, c) in start_vec {
        dist[v] = c;
        pq.push(Node { vertex: v, cost: c });
    }

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
