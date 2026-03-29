use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n],
    }

    let mut a_cnt = vec![0; m];
    let mut b_cnt = vec![0; m];
    for &(a, b) in &ab {
        a_cnt[a] += 1;
        b_cnt[b] += 1;
    }

    println!("{}", (0..m).map(|i| b_cnt[i] - a_cnt[i]).join("\n"));
}
