use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc348/tasks/abc348_e
/// https://atcoder.jp/contests/abc348/editorial/9706
fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n-1],
        c: [usize; n],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    if false {
        solve_rerooting(n, graph, c);
    } else {
        solve_centroid(n, graph, c);
    }
}

/// 全方位木DP
/// https://ei1333.hateblo.jp/entry/2017/04/10/224413
fn solve_rerooting(n: usize, graph: Vec<Vec<usize>>, c: Vec<usize>) {
    let mut subtree_sum_c = vec![0; n];
    let mut subtree_sum_dist = vec![0; n];
    dfs1(&graph, &c, &mut subtree_sum_c, &mut subtree_sum_dist, n, 0);

    let mut f = vec![0; n];
    rerooting(
        &graph,
        &c,
        &subtree_sum_c,
        &subtree_sum_dist,
        &mut f,
        0,
        0,
        n,
        0,
    );

    println!("{}", f.iter().min().unwrap());
}

pub fn dfs1(
    graph: &Vec<Vec<usize>>,
    c: &Vec<usize>,
    subtree_sum_c: &mut Vec<usize>,
    subtree_sum_dist: &mut Vec<usize>,
    par: usize,
    v: usize,
) -> (usize, usize) {
    for &next_v in &graph[v] {
        if next_v == par {
            continue;
        }
        let (subtree_sum_c_next, subtree_sum_dist_next) =
            dfs1(graph, c, subtree_sum_c, subtree_sum_dist, v, next_v);
        subtree_sum_c[v] += subtree_sum_c_next;
        subtree_sum_dist[v] += subtree_sum_dist_next;
    }
    subtree_sum_dist[v] += subtree_sum_c[v];
    subtree_sum_c[v] += c[v];
    (subtree_sum_c[v], subtree_sum_dist[v])
}

fn rerooting(
    graph: &Vec<Vec<usize>>,
    c: &Vec<usize>,
    subtree_sum_c: &Vec<usize>,
    subtree_sum_dist: &Vec<usize>,
    f: &mut Vec<usize>,
    par_sum_c: usize,
    par_sum_dist: usize,
    par: usize,
    v: usize,
) {
    f[v] = subtree_sum_dist[v] + par_sum_dist;
    for &next_v in &graph[v] {
        if next_v == par {
            continue;
        }
        let next_par_sum_c = par_sum_c + subtree_sum_c[v] - subtree_sum_c[next_v];
        let next_par_sum_dist = par_sum_dist + subtree_sum_dist[v] - subtree_sum_dist[next_v]
            + next_par_sum_c
            - subtree_sum_c[next_v];
        rerooting(
            graph,
            c,
            subtree_sum_c,
            subtree_sum_dist,
            f,
            next_par_sum_c,
            next_par_sum_dist,
            v,
            next_v,
        );
    }
}

/// 重心分解
/// https://atcoder.jp/contests/abc291/editorial/5840
fn solve_centroid(n: usize, graph: Vec<Vec<usize>>, c: Vec<usize>) {
    let centroid = find_centroid(&graph, &c);
    let mut dist = vec![0; n];
    tree_dist(&graph, &mut dist, 0, centroid, n);

    println!("{}", (0..n).map(|v| dist[v] * c[v]).sum::<usize>());
}

fn subtree_dfs(
    graph: &Vec<Vec<usize>>,
    weight: &Vec<usize>,
    subtree_weight: &mut Vec<usize>,
    v: usize,
    par: usize,
) -> usize {
    subtree_weight[v] = weight[v];
    for &next_v in &graph[v] {
        if next_v == par {
            continue;
        }
        subtree_weight[v] += subtree_dfs(graph, weight, subtree_weight, next_v, v);
    }
    subtree_weight[v]
}
fn centroid_dfs(
    n: usize,
    graph: &Vec<Vec<usize>>,
    weight_sum: usize,
    subtree_weight: &Vec<usize>,
    v: usize,
    par: usize,
) -> usize {
    let mut is_centroid = true;
    for &next_v in &graph[v] {
        if next_v == par {
            continue;
        }
        let res = centroid_dfs(n, graph, weight_sum, subtree_weight, next_v, v);
        if res != n {
            return res;
        }
        if subtree_weight[next_v] > weight_sum / 2 {
            is_centroid = false;
        }
    }
    if weight_sum - subtree_weight[v] > weight_sum / 2 {
        is_centroid = false;
    }
    if is_centroid {
        v
    } else {
        n
    }
}
pub fn find_centroid(graph: &Vec<Vec<usize>>, weight: &Vec<usize>) -> usize {
    let n = graph.len();
    let weight_sum = weight.iter().sum::<usize>();
    let mut subtree_weight = vec![0; n];
    subtree_dfs(graph, weight, &mut subtree_weight, 0, n);
    centroid_dfs(n, graph, weight_sum, &subtree_weight, 0, n)
}

fn tree_dist(graph: &Vec<Vec<usize>>, dist: &mut Vec<usize>, d: usize, v: usize, par: usize) {
    dist[v] = d;
    for &next_v in &graph[v] {
        if next_v == par {
            continue;
        }
        tree_dist(graph, dist, d + 1, next_v, v);
    }
}
