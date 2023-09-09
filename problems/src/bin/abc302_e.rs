use proconio::input;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut graph_map: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for _ in 0..q {
        input! {
            op: usize,
        }
        match op {
            1 => {
                input! {
                    u: usize,
                    v: usize,
                }
                graph_map
                    .entry(u)
                    .and_modify(|adjacency_set| {
                        adjacency_set.insert(v);
                    })
                    .or_insert(BTreeSet::from([v]));
                graph_map
                    .entry(v)
                    .and_modify(|adjacency_set| {
                        adjacency_set.insert(u);
                    })
                    .or_insert(BTreeSet::from([u]));
                println!("{}", n - graph_map.len());
            }
            2 => {
                input! {
                    u: usize,
                }
                if let Some(adj_nodes) = graph_map.remove(&u) {
                    for &v in &adj_nodes {
                        if let Some(v_set) = graph_map.get_mut(&v) {
                            v_set.remove(&u);
                            if v_set.is_empty() {
                                graph_map.remove(&v);
                            }
                        }
                    }
                }
                println!("{}", n - graph_map.len());
            }
            _ => unreachable!(),
        }
    }
}
