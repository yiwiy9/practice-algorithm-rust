use std::collections::VecDeque;

use proconio::input;

const GRID_SIZE: usize = 1000;
const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/joisc2008/tasks/joisc2008_flu
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   各都市から他の都市を探す話ではなく、各都市の座標から半径 d 以内の整数座標を調べる話にして、
///   JOI 市 P1 から各都市へ流行が始まる最短日 `dist[v]` だけ持てばよい。
/// - それについて、何が分かれば答えになる？
///   各都市の `dist[v]` が分かれば、`k` 日後に流行中である
///   条件 `dist[v] <= k < dist[v] + m` を満たす都市数を数えればよい。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   全都市対の距離計算は不要で、座標が整数格子上かつ `d` が小さいので、
///   各都市について半径 d 以内の整数オフセットだけ調べれば距離 d 以下の候補を漏れなく拾える。
///   感染は辺を 1 日で伝わり、同じ都市で 2 回流行しないので、日ごとの状態全体ではなく最初の到達日だけ持てば十分である。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   `pos[x][y]` で都市の有無を持ち、
///   `dx^2 + dy^2 <= d^2` を満たす整数オフセットを前計算して、
///   BFS 中に各都市の近傍をその場で列挙して `dist` を更新し、最後に `dist[v] in [k-m+1, k]` の都市数を数える。
///
/// 解説AC
///   ある頂点からユークリッド距離がd以下の頂点を O(N^2) を回避して、計算するアルゴリズムが思いつかなかった
///   dが小さいこととある頂点からの次の頂点の数が10以下であることが怪しそうな感じがする
fn main() {
    input! {
        n: usize,
        m: usize,
        d: i64,
        k: usize,
        p: [(usize,usize); n],
    }

    let mut pos = vec![vec![INF; GRID_SIZE]; GRID_SIZE];
    for (i, &(x, y)) in p.iter().enumerate() {
        pos[x][y] = i;
    }

    let d2 = d * d;
    let mut offsets = vec![];
    for dx in -d..=d {
        for dy in -d..=d {
            if dx == 0 && dy == 0 {
                continue;
            }
            if dx * dx + dy * dy <= d2 {
                offsets.push((dx, dy));
            }
        }
    }

    let dist = bfs(&p, &pos, &offsets, k);

    let low = k.saturating_sub(m - 1);
    let ans = dist
        .iter()
        .filter(|&&dist_v| low <= dist_v && dist_v <= k)
        .count();

    println!("{}", ans);
}

fn bfs(
    p: &[(usize, usize)],
    pos: &[Vec<usize>],
    offsets: &[(i64, i64)],
    max_dist: usize,
) -> Vec<usize> {
    let mut dist = vec![INF; p.len()];
    let mut que = VecDeque::new();
    dist[0] = 0;
    que.push_back(0);

    while let Some(v) = que.pop_front() {
        let cur_dist = dist[v];
        if cur_dist == max_dist {
            continue;
        }

        let (x, y) = p[v];
        for &(dx, dy) in offsets {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if nx < 0 || ny < 0 || nx >= GRID_SIZE as i64 || ny >= GRID_SIZE as i64 {
                continue;
            }

            let to = pos[nx as usize][ny as usize];
            if to == INF || dist[to] != INF {
                continue;
            }

            dist[to] = cur_dist + 1;
            que.push_back(to);
        }
    }

    dist
}
