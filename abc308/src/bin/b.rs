use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p: [usize; m+1],
    }

    let mut cnt_map = BTreeMap::new();

    for c_i in &c {
        cnt_map.entry(c_i).and_modify(|cur| *cur += 1).or_insert(1);
    }

    let p0 = p[0];
    let mut price_map = BTreeMap::new();
    for (i, d_i) in d.iter().enumerate() {
        price_map.insert(d_i, p[i + 1]);
    }

    let mut ans = 0;
    for (&key, value) in &cnt_map {
        ans += price_map.get(key).unwrap_or(&p0) * value;
    }

    println!("{}", ans);
}
