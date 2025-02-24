use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc394/tasks/abc394_f
/// https://atcoder.jp/contests/abc394/editorial/12283
///
/// 以下のケースを考えられていなかった
/// - 5つ以上の辺を持つ頂点があったときに、DFSの親の部分木のサイズが必ず他の4頂点よりも大きいとは限らない
///
/// => ans を参照引数で持つようにして、DFSの中で親を取り除いた場合の最大値も更新するようにする
fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    let mut degrees = vec![0; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
        degrees[a] += 1;
        degrees[b] += 1;
    }

    let mut seen = vec![false; n];
    let mut ans = -1;
    for i in 0..n {
        if degrees[i] >= 4 && !seen[i] {
            dfs(&graph, &degrees, &mut seen, &mut ans, i);
        }
    }

    println!("{}", ans);
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    degrees: &Vec<usize>,
    seen: &mut Vec<bool>,
    ans: &mut i32,
    v: usize,
) -> i32 {
    seen[v] = true;

    let mut cnt_vec = vec![];
    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        if degrees[next_v] < 4 {
            cnt_vec.push(1);
            continue;
        }
        cnt_vec.push(dfs(graph, degrees, seen, ans, next_v));
    }

    cnt_vec.sort_by(|a, b| b.cmp(a));

    // 親を取り除いた場合の最大値も更新（根も含む）
    *ans = (*ans).max(cnt_vec.iter().take(4).sum::<i32>() + 1);

    // 親を含める場合は親以外の3頂点分の和を返す
    cnt_vec.iter().take(3).sum::<i32>() + 1
}
