use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    // Functional Graph
    let mut dsu = Dsu::new(n);
    let mut cycle_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for i in 0..n {
        if !dsu.same(i, a[i]) {
            if let Some(cycle_set) = cycle_map.get(&dsu.leader(i)) {
                cycle_map.insert(dsu.leader(a[i]), cycle_set.clone());
            }
            if let Some(cycle_set) = cycle_map.get(&dsu.leader(a[i])) {
                cycle_map.insert(dsu.leader(i), cycle_set.clone());
            }
            dsu.merge(i, a[i]);
            continue;
        }

        // サイクルができたら、サイクル中の要素を記録
        let mut cycle_set = HashSet::from([i]);
        let mut v = a[i];
        while v != i {
            cycle_set.insert(v);
            v = a[v];
        }
        cycle_map.insert(dsu.leader(i), cycle_set);
    }

    let mut memo = vec![0; n];
    let mut ans = 0;
    for i in 0..n {
        ans += dfs(&a, &mut dsu, &cycle_map, &mut memo, i);
    }

    println!("{}", ans);
}

pub fn dfs(
    a: &Vec<usize>,
    dsu: &mut Dsu,
    cycle_map: &HashMap<usize, HashSet<usize>>,
    memo: &mut Vec<usize>,
    v: usize,
) -> usize {
    if let Some(cycle_set) = cycle_map.get(&dsu.leader(v)) {
        if cycle_set.contains(&v) {
            return cycle_set.len();
        }
    } else {
        return 1;
    }

    if memo[v] != 0 {
        return memo[v];
    }

    let cur = dfs(a, dsu, cycle_map, memo, a[v]) + 1;
    memo[v] = cur;

    cur
}
