use num::integer::gcd;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        for j in i + 1..n {
            let mut dx = xy[i].0 - xy[j].0;
            let mut dy = xy[i].1 - xy[j].1;
            let dist = dx * dx + dy * dy;

            // gcd は絶対値で取るのが安心
            let g = gcd(dx.abs(), dy.abs());
            dx /= g;
            dy /= g;

            // 向きを正規化: dx > 0 にする（dx == 0 のときは dy > 0）
            if dx < 0 || (dx == 0 && dy < 0) {
                dx = -dx;
                dy = -dy;
            }

            let key = (dx, dy);

            *map.entry(key)
                .or_insert(BTreeMap::new())
                .entry(dist)
                .or_insert(0) += 1;
        }
    }

    let mut total_cnt = 0;
    let mut double_cnt = 0;
    for key_map in map.values() {
        let mut sum = 0;
        for &cnt in key_map.values() {
            sum += cnt;
            if cnt > 1 {
                double_cnt += cnt * (cnt - 1) / 2;
            }
        }
        total_cnt += sum * (sum - 1) / 2;
    }

    println!("{}", total_cnt - double_cnt / 2);
}
