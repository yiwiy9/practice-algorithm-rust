use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut prev_map = HashMap::new();
    let mut next_map = HashMap::new();
    for i in 1..n {
        prev_map.insert(a[i], a[i - 1]);
        next_map.insert(a[i - 1], a[i]);
    }
    prev_map.insert(a[0], INF);
    next_map.insert(INF, a[0]);
    prev_map.insert(INF, a[n - 1]);
    next_map.insert(a[n - 1], INF);

    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                let next_x = next_map.get(&x).cloned().unwrap();
                next_map.insert(y, next_x);
                prev_map.insert(next_x, y);
                next_map.insert(x, y);
                prev_map.insert(y, x);
            }
            2 => {
                input! {
                    x: usize,
                }
                let prev_x = prev_map.get(&x).cloned().unwrap();
                let next_x = next_map.get(&x).cloned().unwrap();
                next_map.insert(prev_x, next_x);
                prev_map.insert(next_x, prev_x);
                prev_map.remove(&x);
                next_map.remove(&x);
            }
            _ => unreachable!(),
        }
    }

    let mut ans = vec![];
    let mut cur = INF;
    while let Some(&next) = next_map.get(&cur) {
        if next == INF {
            break;
        }
        ans.push(next);
        cur = next;
    }

    println!("{}", ans.iter().join(" "));
}
