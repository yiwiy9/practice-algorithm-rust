use proconio::input;
use std::collections::BTreeMap;

/**
 * https://atcoder.jp/contests/abc166/tasks/abc166_e
 * https://blog.hamayanhamayan.com/entry/2020/05/03/224417
 *
 * ただの式変形
 * j - i = Ai + Aj
 * => i + Ai = j - Aj
 */
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut i_map: BTreeMap<i64, i64> = BTreeMap::new();
    let mut j_map: BTreeMap<i64, i64> = BTreeMap::new();
    for (i, &a_i) in a.iter().enumerate() {
        i_map
            .entry(i as i64 + a_i)
            .and_modify(|cur| *cur += 1)
            .or_insert(1);
        j_map
            .entry(i as i64 - a_i)
            .and_modify(|cur| *cur += 1)
            .or_insert(1);
    }

    let mut ans = 0;
    for x in 1..n {
        if i_map.contains_key(&(x as i64)) && j_map.contains_key(&(x as i64)) {
            ans += i_map.get(&(x as i64)).unwrap() * j_map.get(&(x as i64)).unwrap();
        }
    }

    println!("{}", ans);
}
