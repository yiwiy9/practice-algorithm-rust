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

    for &li in &l {
        if a[li] == n {
            continue;
        }
        if li < k - 1 && a[li] + 1 == a[li + 1] {
            continue;
        }
        a[li] += 1;
    }

    println!("{}", a.iter().join(" "));
}
