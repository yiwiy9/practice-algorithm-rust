use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: usize,
        uv: [(Usize1, Usize1); e],
        q: usize,
        x: [Usize1; q],
    }

    let mut x_set = HashSet::new();
    for &x_i in &x {
        x_set.insert(x_i);
    }

    let mut is_light = (0..n + m)
        .map(|i| if i < n { false } else { true })
        .collect::<Vec<_>>();

    let mut light_count = m;
    let mut dsu = Dsu::new(n + m);
    for (i, &(u, v)) in uv.iter().enumerate() {
        if x_set.contains(&i) {
            continue;
        }

        if is_light[dsu.leader(u)] && !is_light[dsu.leader(v)] {
            light_count += dsu.size(v);
        } else if !is_light[dsu.leader(u)] && is_light[dsu.leader(v)] {
            light_count += dsu.size(u);
        }

        let light = is_light[dsu.leader(u)] || is_light[dsu.leader(v)];
        dsu.merge(u, v);
        is_light[dsu.leader(u)] = light;
    }

    let mut ans = vec![light_count];
    for &x_i in x.iter().rev() {
        let (u, v) = uv[x_i];
        if is_light[dsu.leader(u)] && !is_light[dsu.leader(v)] {
            light_count += dsu.size(v);
        } else if !is_light[dsu.leader(u)] && is_light[dsu.leader(v)] {
            light_count += dsu.size(u);
        }

        let light = is_light[dsu.leader(u)] || is_light[dsu.leader(v)];
        dsu.merge(u, v);
        is_light[dsu.leader(u)] = light;

        ans.push(light_count);
    }

    println!("{}", ans.iter().rev().skip(1).map(|&c| c - m).join("\n"));
}
