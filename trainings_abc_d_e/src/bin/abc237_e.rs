use proconio::{input, marker::Usize1};

/**
 * https://atcoder.jp/contests/abc237/tasks/abc237_e
 * https://atcoder.jp/contests/abc237/editorial/3339
 *
 * - 楽しさの −1 倍をコストとみなすことで、問題文は次のような最短経路の問題に言い換えることができる
 * - ただし、負の辺があるので、ダイクストラ法は使えない
 * - そこで、ポテンシャルを用いてこの問題をダイクストラ法が適用できる形にすることを考える
 * - 高橋君に対して (楽しさ)+(現在いる地点の標高) という量を考える
 * -  (楽しさ)+(現在いる地点の標高) の −1 倍をコストとみなして、ダイクストラ法を適用する
 *
 *
 *
 * ダイクストラ法のよくあるミスと落し方 (snuke先生)
 * https://snuke.hatenablog.com/entry/2021/02/22/102734
 * 負閉路（負のサイクル）がなくても、負の辺がある場合はダイクストラは使えない
 * => 無限ループにはならないけど、O(2^N)になる
 *
 * 同じ理由で、最大ヒープを使ったダイクストラも使えない（たとえ比較を逆にしていても）
 * => A -4-  B  -2-  C  -1- D
 *    |     | |     | |     |
 *    0- E -5 0- F -3 0 -G- 2
 * => この場合、一回目の到達で最大経路が決まらないので、戻るたびにコストの再計算が必要となり、O(2^N)になる
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        let (mut u, mut v) = (u, v);
        if h[u] < h[v] {
            std::mem::swap(&mut u, &mut v);
        }

        graph[u].push((v, 0)); // 楽しさの変化量: -(h[u] - h[v])、ポテンシャルの変化量: -(h[v] - h[u])
        graph[v].push((u, h[u] - h[v])); // 楽しさの変化量: 2*(h[u] - h[v])、ポテンシャルの変化量: -(h[u] - h[v])
    }

    let dist = dijkstra(&graph, 0);

    println!(
        "{}",
        dist.iter()
            .enumerate()
            // 楽しさの変化量 = cost - ポテンシャルの変化量
            .map(|(i, &cost)| -(cost as i64 - (h[0] as i64 - h[i] as i64))) // ポテンシャルの変化量: h[0] - h[i]
            .max()
            .unwrap()
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
pub fn dijkstra(graph: &[Vec<(usize, usize)>], start_v: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![std::usize::MAX; n];
    let mut pq = std::collections::BinaryHeap::new();
    dist[start_v] = 0;
    pq.push(Node {
        vertex: start_v,
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
