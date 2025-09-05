use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        xyz: [(i64, i64, usize); n],
    }

    let z_sum = xyz.iter().map(|&(_, _, z)| z).sum::<usize>();
    let z_win = z_sum / 2 + 1;

    let mut dp = HashMap::new();
    dp.insert(0, 0);
    for &(x, y, z) in &xyz {
        let mut new_dp = dp.clone();
        for (&cur_z, &cur_v) in &dp {
            let next_z = (cur_z + z).min(z_win);
            let next_v = cur_v + ((x + y) / 2 + 1 - x).max(0);
            if let Some(&old_v) = new_dp.get(&next_z) {
                if old_v > next_v {
                    new_dp.insert(next_z, next_v);
                }
            } else {
                new_dp.insert(next_z, next_v);
            }
        }
        dp = new_dp;
    }

    println!("{}", dp[&z_win]);
}
