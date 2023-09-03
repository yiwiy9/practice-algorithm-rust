use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (i, &a_i) in a.iter().enumerate() {
        map.entry(a_i)
            .and_modify(|cur| cur.push(i))
            .or_insert(vec![i]);
    }

    let mut ans = 0;
    for idx_vec in map.values() {
        let mut before_idx = idx_vec[0];
        let mut cnt = 1;
        let mut cur = 0;
        for &idx in idx_vec.iter().skip(1) {
            cur += (idx - before_idx - 1) * cnt;
            ans += cur;
            before_idx = idx;
            cnt += 1;
        }
    }

    println!("{}", ans);
}
