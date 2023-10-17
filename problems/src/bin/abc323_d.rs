use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        sc: [(usize, usize); n],
    }

    let mut map = BTreeMap::new();
    sc.iter().for_each(|&(s, c)| {
        map.entry(s).and_modify(|cur| *cur += c).or_insert(c);
    });

    let mut ans = 0;
    while !map.is_empty() {
        let (s, mut c) = map.pop_first().unwrap();

        if c % 2 == 1 {
            c -= 1;
            ans += 1;
        }

        if c != 0 {
            c /= 2;
            map.entry(s * 2).and_modify(|cur| *cur += c).or_insert(c);
        }
    }

    println!("{}", ans);
}
