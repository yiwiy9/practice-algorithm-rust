use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

fn main() {
    input! {
        m: usize,
        uv: [(Usize1, Usize1); m],
        p: [Usize1; 8],
    }

    let mut graph = vec![vec![]; 9];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut state = vec!['8'; 9];
    for (i, &p_i) in p.iter().enumerate() {
        state[p_i] = std::char::from_digit(i as u32, 10).unwrap();
    }

    let dist = bfs(&graph, &mut state);
    let goal_state = (0..9)
        .map(|i| std::char::from_digit(i as u32, 10).unwrap())
        .collect::<Vec<char>>();

    println!(
        "{}",
        if dist.contains_key(&goal_state) {
            *dist.get(&goal_state).unwrap() as i64
        } else {
            -1
        }
    );
}

pub fn bfs(graph: &Vec<Vec<usize>>, state: &mut Vec<char>) -> BTreeMap<Vec<char>, usize> {
    let mut dist: BTreeMap<Vec<char>, usize> = BTreeMap::new();
    let mut que = std::collections::VecDeque::new();
    dist.insert(state.clone(), 0);
    que.push_back(state.clone());
    while let Some(cur_state) = que.pop_front() {
        let u = find_empty(&cur_state);
        for &v in &graph[u] {
            let mut next_state = cur_state.clone();
            next_state.swap(u, v);
            if dist.contains_key(&next_state) {
                continue;
            }
            dist.insert(next_state.clone(), dist.get(&cur_state).unwrap() + 1);
            que.push_back(next_state.clone());
        }
    }
    dist
}

fn find_empty(state: &Vec<char>) -> usize {
    for (i, &s_i) in state.iter().enumerate() {
        if s_i == '8' {
            return i;
        }
    }
    unreachable!();
}
