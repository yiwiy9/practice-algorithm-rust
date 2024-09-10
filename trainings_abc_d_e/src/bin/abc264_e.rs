use ac_library::Dsu;
use itertools::Itertools as _;
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

    let x_set = x.iter().cloned().collect::<HashSet<_>>();
    let mut dsu = Dsu::new(n + m);
    for (i, &(u, v)) in uv.iter().enumerate() {
        if !x_set.contains(&i) {
            dsu.merge(u, v);
        }
    }

    let mut has_energy_cnt = 0;
    let mut has_energy = vec![false; n + m];
    for i in n..n + m {
        let root = dsu.leader(i);
        if !has_energy[root] {
            has_energy[root] = true;
            has_energy_cnt += dsu.size(root);
        }
        has_energy_cnt -= 1;
    }

    let mut ans = vec![];
    ans.push(has_energy_cnt);

    for &i in x.iter().rev() {
        let (u, v) = uv[i];
        let root_u = dsu.leader(u);
        let root_v = dsu.leader(v);

        if root_u == root_v {
            ans.push(has_energy_cnt);
            continue;
        }

        if !has_energy[root_u] && has_energy[root_v] {
            has_energy_cnt += dsu.size(root_u);
        }
        if has_energy[root_u] && !has_energy[root_v] {
            has_energy_cnt += dsu.size(root_v);
        }

        dsu.merge(root_u, root_v);
        // マージ後の root を再取得しない場合
        has_energy[root_u] |= has_energy[root_v];
        has_energy[root_v] = has_energy[root_u];
        // // マージ後の root は変わるので再取得 ← 嘘。↑でも通る
        // has_energy[dsu.leader(root_u)] = has_energy[root_u] | has_energy[root_v];
        ans.push(has_energy_cnt);
    }

    println!("{}", ans.iter().rev().skip(1).join("\n"));
}
