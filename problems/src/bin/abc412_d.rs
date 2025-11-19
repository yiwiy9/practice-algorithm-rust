use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut edge_set = HashSet::new();
    for &(a, b) in &ab {
        edge_set.insert((a.min(b), a.max(b)));
    }

    let mut ans = INF;
    for order in (0..n).permutations(n) {
        let mut add_cnt = 0;
        let mut use_cnt = 0;

        for i in 0..n - 1 {
            if edge_set.contains(&(order[i].min(order[i + 1]), order[i].max(order[i + 1]))) {
                use_cnt += 1;
            } else {
                add_cnt += 1;
            }
        }
        if edge_set.contains(&(order[0].min(order[n - 1]), order[0].max(order[n - 1]))) {
            use_cnt += 1;
        } else {
            add_cnt += 1;
        }
        ans = ans.min(add_cnt + (m - use_cnt));

        if n >= 6 {
            for connected_cnt in 3..=n / 2 {
                let mut add_cnt = 0;
                let mut use_cnt = 0;

                for i in 0..connected_cnt - 1 {
                    if edge_set.contains(&(order[i].min(order[i + 1]), order[i].max(order[i + 1])))
                    {
                        use_cnt += 1;
                    } else {
                        add_cnt += 1;
                    }
                }
                if edge_set.contains(&(
                    order[0].min(order[connected_cnt - 1]),
                    order[0].max(order[connected_cnt - 1]),
                )) {
                    use_cnt += 1;
                } else {
                    add_cnt += 1;
                }

                for i in connected_cnt..n - 1 {
                    if edge_set.contains(&(order[i].min(order[i + 1]), order[i].max(order[i + 1])))
                    {
                        use_cnt += 1;
                    } else {
                        add_cnt += 1;
                    }
                }
                if edge_set.contains(&(
                    order[connected_cnt].min(order[n - 1]),
                    order[connected_cnt].max(order[n - 1]),
                )) {
                    use_cnt += 1;
                } else {
                    add_cnt += 1;
                }

                ans = ans.min(add_cnt + (m - use_cnt));
            }
        }
    }

    println!("{}", ans);
}
