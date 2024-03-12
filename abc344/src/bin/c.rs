use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    }

    let mut set = BTreeSet::new();
    for &a_i in &a {
        for &b_i in &b {
            for &c_i in &c {
                set.insert(a_i + b_i + c_i);
            }
        }
    }

    for &x_i in &x {
        println!("{}", if set.contains(&x_i) { "Yes" } else { "No" });
    }
}
