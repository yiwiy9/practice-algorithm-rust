use proconio::input;
use std::collections::BTreeMap;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        x: usize,
        vac: [(usize, usize, usize); n],
    }

    let mut ac_1 = vec![];
    let mut ac_2 = vec![];
    let mut ac_3 = vec![];
    for &(v, a, c) in &vac {
        match v {
            1 => ac_1.push((a, c)),
            2 => ac_2.push((a, c)),
            3 => ac_3.push((a, c)),
            _ => unreachable!(),
        }
    }

    let map_1 = dp(&ac_1, x);
    let map_2 = dp(&ac_2, x);
    let map_3 = dp(&ac_3, x);

    let mut left = 0;
    let mut right = INF;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let c_1 = map_1.range(mid..).next().unwrap_or((&INF, &INF)).1;
        let c_2 = map_2.range(mid..).next().unwrap_or((&INF, &INF)).1;
        let c_3 = map_3.range(mid..).next().unwrap_or((&INF, &INF)).1;

        if c_1 + c_2 + c_3 <= x {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}

fn dp(ac: &[(usize, usize)], x: usize) -> BTreeMap<usize, usize> {
    let mut dp = vec![0; x + 1];
    for i in 0..ac.len() {
        let (a, c) = ac[i];
        let mut next_dp = vec![0; x + 1];
        for j in 0..=x {
            next_dp[j] = next_dp[j].max(dp[j]);
            if j + c <= x {
                next_dp[j + c] = next_dp[j + c].max(dp[j] + a);
            }
        }
        dp = next_dp;
    }

    let mut map = BTreeMap::new();
    map.insert(0, 0);
    let mut before_a = 0;
    for (x_i, &a_i) in dp.iter().enumerate() {
        if a_i > before_a {
            map.insert(a_i, x_i);
            before_a = a_i;
        }
    }
    map
}
