use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let a_set: BTreeSet<usize> = a.iter().cloned().collect();

    let mut ans = 0;
    for i in 0..=2000 {
        if !a_set.contains(&i) {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}
