use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        d: usize,
        n: usize,
        m: usize,
        dd: [usize; n-1],
        k: [usize; m],
    }

    let mut d_set = BTreeSet::from_iter(dd);
    d_set.insert(0);
    d_set.insert(d);

    let mut ans: usize = 0;
    for &k_i in &k {
        let back = *d_set.range(..=k_i).next_back().unwrap();
        let forward = *d_set.range(k_i..).next().unwrap();
        ans += (k_i - back).min(forward - k_i);
    }

    println!("{}", ans);
}
