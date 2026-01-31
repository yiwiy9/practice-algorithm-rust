use proconio::{input, marker::Usize1};
use superslice::*;

/// https://atcoder.jp/contests/joi2024ho/tasks/joi2024ho_b
/// https://www2.ioi-jp.org/joi/2023/2024-ho/2024-ho-t2-review.pdf
///
/// この問題は「長さ L の辺を 1 本だけ追加したとき、S→T を K 以下にできる追加位置の数」を数える。
/// 追加後の最短路は
/// - 追加した辺を使わない（もともとの最短路）
/// - 追加した辺を 1 回使う
/// のどちらかしかない。
///
/// まず `dist(S,T) <= K` なら、追加した辺を使う必要がないので、どこに辺を張っても条件を満たす。
/// よって答えは全ペア `N*(N-1)/2` になる。
///
/// 以降は `dist(S,T) > K` を考える。このとき条件を満たすには、追加した辺を必ず使う必要がある。
/// 追加する辺を (u, v) とすると、S→T の候補は
/// - S→u + L + v→T
/// - S→v + L + u→T
/// の 2 通りだけである。
///
/// 「P: S→u + L + v→T <= K」「Q: S→v + L + u→T <= K」とおくと、
/// 本来は「P または Q」を満たす (u,v) を数えればよい。
/// しかし `dist(S,T) > K` の状況では、P と Q を同時に満たす (u,v) は存在しない。
/// もし両方満たすとすると
///     (S→u) + (v→T) <= K-L
///     (S→v) + (u→T) <= K-L
/// を足し合わせることで、三角不等式（S→T <= S→u + u→T など）と矛盾し、
/// 結果として `dist(S,T) <= K` が導かれてしまうためである。
/// したがって「P または Q」は「P を満たす順序付きペア (u,v)」を数えるだけに帰着できる。
///
/// すると (u を固定したとき) 必要なのは
///     v→T <= K - (S→u + L)
/// を満たす v の個数だけであり、これは v 依存の値 `dist(v,T)` の閾値判定になっている。
/// よって `dist(v,T)` をソートして二分探索すれば、各 u を `O(log N)` で数え上げられる。
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
        l: usize,
        k: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let from_s = dijkstra(&graph, s);
    let from_t = dijkstra(&graph, t);

    // 最初から条件を満たすなら、どの辺を足してもOK
    if from_s[t] <= k {
        println!("{}", n * (n - 1) / 2);
        return;
    }

    let mut from_t_sorted = from_t.clone();
    from_t_sorted.sort();

    let mut ans: usize = 0;
    for v in 0..n {
        let from_s_l = from_s[v] + l;
        if from_s_l > k {
            continue;
        }
        let from_s_cnt = from_t_sorted.upper_bound(&(k - from_s_l));
        ans += from_s_cnt;
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
