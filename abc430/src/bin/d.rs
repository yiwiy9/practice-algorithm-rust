use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
    }

    let mut d_vec = vec![0; n + 1];
    let mut map = BTreeMap::new();
    map.insert(0, 0);

    let mut ans = 0;
    for (i, &x_i) in x.iter().enumerate() {
        let mut cur_d = 1 << 30;
        if let Some((&prev_num, &prev_i)) = map.range(..x_i).next_back() {
            let prev_d = x_i - prev_num;
            if i == 0 || prev_d < d_vec[prev_i] {
                ans -= d_vec[prev_i];
                d_vec[prev_i] = prev_d;
                ans += d_vec[prev_i];
            }

            cur_d = cur_d.min(prev_d);
        }

        if let Some((&next_num, &next_i)) = map.range(x_i..).next() {
            let next_d = next_num - x_i;
            if i == 0 || next_d < d_vec[next_i] {
                ans -= d_vec[next_i];
                d_vec[next_i] = next_d;
                ans += d_vec[next_i];
            }

            cur_d = cur_d.min(next_d);
        }

        d_vec[i + 1] = cur_d;
        ans += d_vec[i + 1];
        map.insert(x_i, i + 1);

        println!("{}", ans);
    }
}
