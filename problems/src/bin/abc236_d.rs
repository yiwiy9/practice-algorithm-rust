use proconio::input;
use std::collections::BTreeMap;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
    }

    let mut map = BTreeMap::new();
    for i in 1..=2 * n - 1 {
        for j in i + 1..=2 * n {
            input! {
                a: usize,
            }
            map.insert((i, j), a);
        }
    }

    let mut used = vec![false; 2 * n + 1];
    let ans = rec(n, &map, &mut used, INF);

    println!("{}", ans);
}

fn rec(n: usize, map: &BTreeMap<(usize, usize), usize>, used: &mut Vec<bool>, cur: usize) -> usize {
    if used.iter().skip(1).all(|&x| x) {
        return cur;
    }

    let mut l = 0;
    for i in 1..=2 * n {
        if !used[i] {
            l = i;
            break;
        }
    }
    used[l] = true;

    let mut res = 0;
    for j in l + 1..=2 * n {
        if used[j] {
            continue;
        }

        let next = if cur == INF {
            map[&(l, j)]
        } else {
            cur ^ map[&(l, j)]
        };

        used[j] = true;
        res = res.max(rec(n, map, used, next));
        used[j] = false;
    }

    used[l] = false;
    res
}
