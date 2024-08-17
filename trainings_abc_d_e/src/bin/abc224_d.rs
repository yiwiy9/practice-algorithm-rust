use proconio::{input, marker::Usize1};
use std::collections::{HashMap, VecDeque};
const EMPTY: usize = 8;

fn main() {
    input! {
        m: usize,
        uv: [(Usize1, Usize1); m],
        p: [Usize1; 8],
    }

    let mut graph = vec![vec![]; 9];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut state = vec![EMPTY; 9];
    for i in 0..8 {
        state[p[i]] = i;
    }

    let memo = bfs(&graph, &state);

    if let Some(ans) = memo.get(&[0, 1, 2, 3, 4, 5, 6, 7, 8].to_vec()) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

pub fn bfs(graph: &Vec<Vec<usize>>, init_state: &Vec<usize>) -> HashMap<Vec<usize>, usize> {
    let mut memo = HashMap::new();
    let mut que = VecDeque::new();

    memo.insert(init_state.clone(), 0);
    que.push_back(init_state.clone());
    while let Some(state) = que.pop_front() {
        let empty = state.iter().position(|&x| x == EMPTY).unwrap();
        for &v in &graph[empty] {
            let mut new_state = state.clone();
            new_state.swap(empty, v);
            if memo.contains_key(&new_state) {
                continue;
            }
            memo.insert(new_state.clone(), memo[&state] + 1);
            que.push_back(new_state);
        }
    }

    memo
}
