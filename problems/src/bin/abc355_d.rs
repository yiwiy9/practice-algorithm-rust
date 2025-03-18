use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize,usize); n],
    }
    lr.sort();

    let l_vec = lr.iter().map(|&(l, _)| l).collect::<Vec<_>>();

    let mut ans = 0;
    for (i, &(_, r)) in lr.iter().enumerate() {
        ans += l_vec.upper_bound(&r) - 1 - i;
    }

    println!("{}", ans);
}
