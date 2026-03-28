use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        t: [usize; n],
        abd: [(Usize1,Usize1,usize); m],
    }
    let mut graph = vec![vec![]; n];
    for &(a, b, d) in &abd {
        graph[a].push((b, d));
        graph[b].push((a, d));
    }

    let dist = dijkstra(&graph, &t, x);

    println!(
        "{}",
        dist[n - 1][0]
            .iter()
            .min()
            .unwrap()
            .min(dist[n - 1][1].iter().min().unwrap())
    );
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    is_hot: usize,
    time: usize,
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
pub fn dijkstra(graph: &[Vec<(usize, usize)>], t: &[usize], x: usize) -> Vec<Vec<Vec<usize>>> {
    let inf: usize = 1 << 60;
    let n = graph.len();
    let mut dist = vec![vec![vec![inf; x + 1]; 2]; n];
    let mut pq = std::collections::BinaryHeap::new();
    dist[0][0][x] = 0;
    pq.push(Node {
        vertex: 0,
        is_hot: 0,
        time: x,
        cost: 0,
    });
    while let Some(Node {
        vertex,
        is_hot,
        time,
        cost,
    }) = pq.pop()
    {
        if dist[vertex][is_hot][time] < cost {
            continue;
        }

        for &(next_vertex, edge_cost) in &graph[vertex] {
            let mut next_time = time.saturating_sub(edge_cost);
            let mut next_is_hot = is_hot;
            match t[next_vertex] {
                0 => {
                    if next_time > 0 && is_hot == 1 {
                        continue;
                    }
                    next_is_hot = 0;
                    next_time = x;
                }
                1 => {}
                2 => {
                    if next_time > 0 && is_hot == 0 {
                        continue;
                    }
                    next_is_hot = 1;
                    next_time = x;
                }
                _ => unreachable!(),
            }

            let new_cost = cost + edge_cost;
            if new_cost < dist[next_vertex][next_is_hot][next_time] {
                dist[next_vertex][next_is_hot][next_time] = new_cost;
                pq.push(Node {
                    vertex: next_vertex,
                    is_hot: next_is_hot,
                    time: next_time,
                    cost: new_cost,
                });
            }
        }
    }

    dist
}
