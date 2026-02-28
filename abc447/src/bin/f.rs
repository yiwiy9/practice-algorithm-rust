use proconio::{input, marker::Usize1};

fn main() {
    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            n: usize,
            ab: [(Usize1,Usize1); n-1],
        }

        let mut graph = vec![vec![]; n];
        for &(a, b) in &ab {
            graph[a].push(b);
            graph[b].push(a);
        }

        let mut ans = 1;
        dfs(&graph, &mut ans, n, 0);

        println!("{}", ans);
    }
}

pub fn dfs(graph: &Vec<Vec<usize>>, max_cnt: &mut usize, parent_v: usize, v: usize) -> usize {
    let mut first = 0;
    let mut second = 0;
    for &next_v in &graph[v] {
        if next_v == parent_v {
            continue;
        }
        let res = dfs(graph, max_cnt, v, next_v);
        if res >= first {
            second = first;
            first = res;
        } else if res >= second {
            second = res;
        }
    }

    if graph[v].len() <= 2 {
        0
    } else if graph[v].len() >= 4 {
        if *max_cnt < first + second + 1 {
            *max_cnt = first + second + 1;
        }
        first + 1
    } else {
        if *max_cnt < first + 1 {
            *max_cnt = first + 1;
        }
        1
    }
}
