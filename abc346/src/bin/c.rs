use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut set = BTreeSet::new();
    for a_i in a {
        set.insert(a_i);
    }

    let mut ans = k * (k + 1) / 2;
    for &i in set.iter() {
        if i <= k {
            ans -= i;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
