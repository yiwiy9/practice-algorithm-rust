use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

const INF: usize = 1 << 30;

/**
 * https://atcoder.jp/contests/abc344/tasks/abc344_e
 * https://atcoder.jp/contests/abc344/editorial/9487
 *
 * 双方向連結リストを使う問題
 * 完璧なデータ構造を実装しなくても、本質を理解していればBTreeMapで前後を管理するだけで良いことがわかる
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut prev_map = BTreeMap::new();
    let mut next_map = BTreeMap::new();

    prev_map.insert(INF, a[n - 1]);
    next_map.insert(INF, a[0]);
    for (i, &a_i) in a.iter().enumerate() {
        prev_map.insert(a_i, if i > 0 { a[i - 1] } else { INF });
        next_map.insert(a_i, if i < n - 1 { a[i + 1] } else { INF });
    }

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

                let next_x = *next_map.get(&x).unwrap();

                prev_map.insert(next_x, y);
                next_map.insert(y, next_x);

                prev_map.insert(y, x);
                next_map.insert(x, y);
            }
            2 => {
                input! {
                    x: usize,
                }

                let prev_x = *prev_map.get(&x).unwrap();
                let next_x = *next_map.get(&x).unwrap();

                prev_map.insert(next_x, prev_x);
                next_map.insert(prev_x, next_x);

                prev_map.remove(&x);
                next_map.remove(&x);
            }
            _ => unreachable!(),
        }
    }

    let mut ans = vec![];
    let mut x = *next_map.get(&INF).unwrap();
    loop {
        if x == INF {
            break;
        }
        ans.push(x);
        x = *next_map.get(&x).unwrap();
    }

    println!("{}", ans.iter().join(" "));
}
