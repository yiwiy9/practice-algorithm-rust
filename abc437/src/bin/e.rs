use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        xy: [(usize,usize); n],
    }

    let mut graph: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); n + 1];
    let mut first_v = (0..n + 1).collect_vec();
    let mut first_v_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (i, &(x, y)) in xy.iter().enumerate() {
        if let Some(&first_i) = graph[first_v[x]].get(&y) {
            first_v[i + 1] = first_i;
            first_v_map.entry(first_i).or_default().push(i + 1);
        } else {
            graph[first_v[x]].insert(y, i + 1);
            first_v_map.entry(i + 1).or_default().push(i + 1);
        }
    }

    first_v_map.iter_mut().for_each(|(_, v)| v.sort());

    let mut ans = vec![];
    dfs(&graph, &first_v_map, &mut vec![false; n + 1], &mut ans, 0);

    println!("{}", ans.iter().join(" "));
}

pub fn dfs(
    graph: &Vec<BTreeMap<usize, usize>>,
    first_v_map: &BTreeMap<usize, Vec<usize>>,
    seen: &mut Vec<bool>,
    ans: &mut Vec<usize>,
    v: usize,
) {
    seen[v] = true;

    for (_, &next_first_v) in &graph[v] {
        if seen[next_first_v] {
            continue;
        }
        for &next_v in &first_v_map[&next_first_v] {
            ans.push(next_v);
        }
        dfs(graph, first_v_map, seen, ans, next_first_v);
    }
}
