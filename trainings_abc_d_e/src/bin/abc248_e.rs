use num_integer::gcd;
use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i64,i64); n],
    }

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut used = HashSet::new();
    let mut ans = 0;
    for (i, &(x1, y1)) in xy.iter().enumerate() {
        let mut trend_map = HashMap::new();
        for (j, &(x2, y2)) in xy.iter().enumerate() {
            if i == j {
                continue;
            }
            let mut dx = x2 - x1;
            let mut dy = y2 - y1;
            if dx < 0 {
                dx = -dx;
                dy = -dy;
            }
            (dx, dy) = match (dx, dy) {
                (0, _) => (0, 1),
                (_, 0) => (1, 0),
                (dx, dy) => {
                    let g = gcd(dx.abs(), dy.abs());
                    (dx / g, dy / g)
                }
            };
            if !used.contains(&(x2, y2, dx, dy)) {
                *trend_map.entry((dx, dy)).or_insert(0) += 1;
                used.insert((x2, y2, dx, dy));
            }
        }
        for (&(dx, dy), &cnt) in &trend_map {
            if cnt >= k - 1 {
                ans += 1;
            }
            used.insert((x1, y1, dx, dy));
        }
    }

    println!("{}", ans);
}
