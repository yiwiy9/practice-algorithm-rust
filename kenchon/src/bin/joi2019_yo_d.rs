use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = BTreeMap::new();
    for (i, &a_i) in a.iter().enumerate() {
        if a_i == 0 {
            continue;
        }
        map.entry(a_i).or_insert(vec![]).push(i);
    }

    let mut ans: usize = 0;
    let mut cur = 0; // 逆に全てが海の状態からスタート
    let mut is_land = vec![false; n];
    for indexes in map.values().rev() {
        for &i in indexes {
            let left = if i > 0 { is_land[i - 1] } else { false };
            let right = if i + 1 < n { is_land[i + 1] } else { false };

            if left && right {
                cur -= 1;
            } else if !left && !right {
                cur += 1;
            }

            is_land[i] = true;
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
