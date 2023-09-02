use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let mut dp = BTreeMap::new();
    dp.insert(x, 1);

    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        }

        let mut new_dp = BTreeMap::new();
        for (x, &cnt) in &dp {
            for a_i in &a {
                if x % a_i != 0 || x / a_i == 0 {
                    continue;
                }
                new_dp
                    .entry(x / a_i)
                    .and_modify(|cur| *cur += cnt)
                    .or_insert(cnt);
            }
        }
        dp = new_dp;
    }

    println!("{}", dp.get(&1).unwrap_or(&0));
}
