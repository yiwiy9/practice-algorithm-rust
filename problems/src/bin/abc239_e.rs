use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; n],
        ab: [(Usize1,Usize1); n-1],
        vk: [(Usize1,Usize1); q],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut kth_vec = vec![vec![]; n];
    dfs(&graph, &x, &mut vec![false; n], &mut kth_vec, 0);

    for &(v, k) in &vk {
        println!("{}", kth_vec[v][k]);
    }
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    x: &Vec<usize>,
    seen: &mut Vec<bool>,
    kth_vec: &mut Vec<Vec<usize>>,
    v: usize,
) {
    seen[v] = true;

    let mut merged = vec![x[v]];
    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, x, seen, kth_vec, next_v);
        kth_vec[next_v].iter().for_each(|&cnt| merged.push(cnt));
    }
    merged.sort_by(|a, b| b.cmp(a));

    merged.iter().take(20).for_each(|&cnt| kth_vec[v].push(cnt))
}
