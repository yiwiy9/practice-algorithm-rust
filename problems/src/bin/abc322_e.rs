use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
    }

    let mut c = vec![];
    let mut a = vec![];
    for _ in 0..n {
        input! {
            c_i: usize,
            a_i: [usize; k],
        }
        c.push(c_i);
        a.push(a_i);
    }

    let mut dp = BTreeMap::new();
    dp.insert(vec![0; k], 0);
    for i in 0..n {
        let mut next_dp = dp.clone();

        for (state, &cost) in &dp {
            let mut next_state = state.clone();
            for (j, &a_j) in a[i].iter().enumerate() {
                next_state[j] = p.min(state[j] + a_j);
            }

            let next_cost = cost + c[i];

            if let Some(&cur_cost) = next_dp.get(&next_state) {
                next_dp.insert(next_state, cur_cost.min(next_cost));
            } else {
                next_dp.insert(next_state, next_cost);
            }
        }

        dp = next_dp;
    }

    println!(
        "{}",
        if let Some(&ans) = dp.get(&vec![p; k]) {
            ans as i64
        } else {
            -1
        }
    );
}
