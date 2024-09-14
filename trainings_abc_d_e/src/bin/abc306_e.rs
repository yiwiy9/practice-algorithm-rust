use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

/**
 * https://atcoder.jp/contests/abc306/tasks/abc306_e
 * https://atcoder.jp/contests/abc306/editorial/6607
 * > 今回はデータ構造として multiset(多重集合を管理するデータ構造) を用います。
 * ↓
 * Rust では multiset は標準ライブラリには存在しないため、BTreeMap を使って頑張る
 * mapを２つ使うのがポイント
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        xy: [(Usize1, usize); q],
    }

    let mut a = vec![0; n];
    let mut l_sum = 0;

    let mut l_map = BTreeMap::new();
    l_map.insert(0, k);

    let mut s_map = BTreeMap::new();
    s_map.insert(0, n - k);

    for &(x, y) in &xy {
        let before_a = a[x];

        if before_a >= *l_map.iter().next().unwrap().0 {
            l_sum -= before_a;

            if Some(&1) == l_map.get(&before_a) {
                l_map.remove(&before_a);
            } else {
                l_map.entry(before_a).and_modify(|e| *e -= 1);
            }

            let s_max = *s_map.iter().next_back().unwrap().0;

            if y >= s_max {
                l_sum += y;
                *l_map.entry(y).or_insert(0) += 1;
            } else {
                if Some(&1) == s_map.get(&s_max) {
                    s_map.remove(&s_max);
                } else {
                    s_map.entry(s_max).and_modify(|e| *e -= 1);
                }

                l_sum += s_max;
                *l_map.entry(s_max).or_insert(0) += 1;
                *s_map.entry(y).or_insert(0) += 1;
            }
        } else {
            if Some(&1) == s_map.get(&before_a) {
                s_map.remove(&before_a);
            } else {
                s_map.entry(before_a).and_modify(|e| *e -= 1);
            }

            let l_min = *l_map.iter().next().unwrap().0;
            if y <= l_min {
                *s_map.entry(y).or_insert(0) += 1;
            } else {
                if Some(&1) == l_map.get(&l_min) {
                    l_map.remove(&l_min);
                } else {
                    l_map.entry(l_min).and_modify(|e| *e -= 1);
                }

                l_sum += y - l_min;
                *l_map.entry(y).or_insert(0) += 1;
                *s_map.entry(l_min).or_insert(0) += 1;
            }
        }

        a[x] = y;
        println!("{}", l_sum);
    }
}
