use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(i64, i64); n],
    }

    let mut sum_l = lr.iter().map(|&(l, _)| l).sum::<i64>();
    let mut sum_r = lr.iter().map(|&(_, r)| r).sum::<i64>();
    if sum_l > 0 || sum_r < 0 {
        println!("No");
        return;
    }

    let mut x = lr.iter().map(|&(l, _)| l).collect::<Vec<i64>>();
    let mut sum_x = sum_l;
    for i in 0..n {
        let d = (lr[i].1 - lr[i].0).min(sum_x.abs());
        x[i] += d;
        sum_x += d;
    }

    println!("Yes");
    println!("{}", x.iter().join(" "));
}
