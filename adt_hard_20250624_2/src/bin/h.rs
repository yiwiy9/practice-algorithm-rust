use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [Usize1; n-1],
        xy: [(Usize1,i64); m],
    }

    let mut graph = vec![vec![]; n];
    for (i, &p_i) in p.iter().enumerate() {
        graph[p_i].push(i + 1);
    }

    let mut seen = vec![false; n];
    let mut order = vec![];
    for v in 0..n {
        if seen[v] {
            continue;
        }
        dfs_1(&graph, &mut seen, &mut order, v);
    }
    order.reverse();

    let mut insurance_map = HashMap::new();
    for (x, y) in xy {
        insurance_map
            .entry(x)
            .and_modify(|e| {
                if *e < y + 1 {
                    *e = y + 1;
                }
            })
            .or_insert(y + 1);
    }

    let mut insurance = vec![-1; n];
    for v in order {
        if insurance[v] != -1 {
            continue;
        }
        dfs_2(&graph, &insurance_map, &mut insurance, v, 0);
    }

    println!("{}", insurance.iter().filter(|&&x| x > 0).count());
}

pub fn dfs_1(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, order: &mut Vec<usize>, v: usize) {
    seen[v] = true;

    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs_1(graph, seen, order, next_v);
    }

    // トポロジカルソート順
    order.push(v);
}

pub fn dfs_2(
    graph: &Vec<Vec<usize>>,
    insurance_map: &HashMap<usize, i64>,
    insurance: &mut Vec<i64>,
    v: usize,
    ins: i64,
) {
    let cur_ins = insurance_map.get(&v).cloned().unwrap_or(0).max(ins - 1);
    insurance[v] = cur_ins;
    for &next_v in &graph[v] {
        dfs_2(graph, insurance_map, insurance, next_v, cur_ins);
    }
}
