use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [Usize1; q],
    }

    for &l_i in &l {
        if a[l_i] == n {
            continue;
        }
        if l_i < k - 1 && a[l_i] + 1 == a[l_i + 1] {
            continue;
        }
        a[l_i] += 1;
    }

    println!("{}", a.iter().join(" "));
}
