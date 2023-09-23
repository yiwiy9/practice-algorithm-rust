use proconio::input;
use std::collections::BTreeMap;

const MAX: usize = 1_000_001;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt_map = BTreeMap::new();
    for &a_i in &a {
        cnt_map.entry(a_i).and_modify(|cur| *cur += 1).or_insert(1);
    }

    let mut ans = n;
    let mut divisible = vec![false; MAX];
    for (&a_i, &cnt) in cnt_map.iter() {
        if cnt > 1 {
            ans -= cnt;
        } else if divisible[a_i] {
            ans -= 1;
            continue;
        }

        divisible[a_i] = true;
        let mut a_j = a_i * 2;
        while a_j < MAX {
            divisible[a_j] = true;
            a_j += a_i;
        }
    }

    println!("{}", ans);
}
