use proconio::input;
use std::collections::HashMap;

const INF: usize = 1 << 60;
const INIT_KEY: (usize, usize, usize) = (0, INF, 0);

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }

    let mut dp = HashMap::new();
    dp.insert(INIT_KEY, 0);
    for &a_i in &a {
        let mut next_dp: HashMap<(usize, usize, usize), usize> = HashMap::new();
        let mut init_value_min = INF;

        for (&(cnt, min, max), &value) in &dp {
            let next_cnt = cnt + 1;
            let next_min = min.min(a_i);
            let next_max = max.max(a_i);

            init_value_min = init_value_min.min(value + k + next_cnt * (next_max - next_min));

            if next_cnt < m {
                next_dp.insert((next_cnt, next_min, next_max), value);
            }
        }

        next_dp.insert(INIT_KEY, init_value_min);
        dp = next_dp;
    }

    println!("{}", dp.get(&INIT_KEY).unwrap());
}
