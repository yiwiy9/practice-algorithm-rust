use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/joi2011ho/tasks/joi2011ho3
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   各町の最寄りモール距離 dist を持つ。辺上の距離は両端の dist だけで決まる。
/// - それについて、何が分かれば答えになる？
///   各町と各辺上での最大距離が分かれば答え。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   最短路経路自体は不要。辺上は min(dist[a]+x, dist[b]+l-x) なので両端だけ見ればよい。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   多始点 Dijkstra 後、各辺で diff=|dist[a]-dist[b]| を見て、diff<l なら内部候補を計算して最大を取る。
///
/// AC: 45分
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        abl: [(Usize1, Usize1,i64); m],
        s: [Usize1; k],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, l) in &abl {
        graph[a].push((b, l));
        graph[b].push((a, l));
    }
    let dist = dijkstra(&graph, &s);

    let mut ans = *dist.iter().max().unwrap();
    for &(a, b, l) in &abl {
        let diff = (dist[a] - dist[b]).abs();
        if diff < l {
            let max_dist = dist[a].max(dist[b]);
            let road_dist = (l - diff + 1) / 2;
            ans = ans.max(max_dist + road_dist);
        }
    }

    println!("{}", ans);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    cost: i64,
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
pub fn dijkstra(graph: &[Vec<(usize, i64)>], s: &[usize]) -> Vec<i64> {
    let inf: i64 = 1 << 60;
    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    for &s_i in s {
        dist[s_i] = 0;
        pq.push(Node {
            vertex: s_i,
            cost: 0,
        });
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
