use proconio::input;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: [[usize; n]; n],
    }

    let mut graph_car = vec![vec![]; n];
    for i in 0..n {
        for j in i + 1..n {
            graph_car[i].push((j, d[i][j] * a));
            graph_car[j].push((i, d[i][j] * a));
        }
    }
    let car_dist = dijkstra_car(&graph_car);

    let mut graph_train = vec![vec![]; n];
    for i in 0..n {
        for j in i + 1..n {
            graph_train[i].push((j, d[i][j] * b + c));
            graph_train[j].push((i, d[i][j] * b + c));
        }
    }
    let train_dist = dijkstra_train(&graph_train);

    let mut ans = INF;
    for i in 0..n {
        ans = ans.min(car_dist[i] + train_dist[i]);
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
pub fn dijkstra_car(graph: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![std::usize::MAX; n];
    let mut pq = std::collections::BinaryHeap::new();
    dist[0] = 0;
    pq.push(Node { vertex: 0, cost: 0 });
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
pub fn dijkstra_train(graph: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![std::usize::MAX; n];
    let mut pq = std::collections::BinaryHeap::new();
    dist[n - 1] = 0;
    pq.push(Node {
        vertex: n - 1,
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
