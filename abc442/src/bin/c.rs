use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut c = vec![0; n];
    for &(a, b) in &ab {
        c[a] += 1;
        c[b] += 1;
    }

    let mut ans = vec![];
    for &cnt in &c {
        let candidate = n - cnt - 1;
        ans.push(if candidate >= 3 {
            candidate * (candidate - 1) * (candidate - 2) / 6
        } else {
            0
        });
    }

    println!("{}", ans.iter().join(" "));
}
