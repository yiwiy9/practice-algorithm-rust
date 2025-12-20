use proconio::input;
use std::collections::BTreeMap;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize,usize); n],
    }

    ab.sort_by(|a, b| a.1.cmp(&b.1));

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    for &(a, b) in &ab {
        *map.entry(a + b).or_default() += 1;
    }

    let mut ans: usize = INF;
    for &(a, b) in &ab {
        let num = map.get_mut(&(a + b)).unwrap();
        *num -= 1;
        if *num == 0 {
            map.remove(&(a + b));
        }

        if let Some((&k1, &v1)) = map.iter().next() {
            if v1 >= 2 {
                ans = ans.min(a + k1 + k1);
            } else if let Some((&k2, _)) = map.iter().nth(1) {
                ans = ans.min(a + k1 + k2);
            }
        }
    }

    println!("{}", ans);
}
