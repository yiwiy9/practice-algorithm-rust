use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
        b: [usize; m],
        cd: [(Usize1,Usize1); l],
    }

    let mut c: Vec<(usize, usize)> = b.iter().enumerate().map(|(i, &b_i)| (b_i, i)).collect();
    c.sort_by(|a, b| b.cmp(a));
    let mut set = BTreeSet::new();
    for cd_i in cd {
        set.insert(cd_i);
    }

    let mut ans = 0;
    for (i, &a_i) in a.iter().enumerate() {
        for &(b_j, j) in &c {
            if !set.contains(&(i, j)) {
                ans = ans.max(a_i + b_j);
                break;
            }
        }
    }

    println!("{}", ans);
}
