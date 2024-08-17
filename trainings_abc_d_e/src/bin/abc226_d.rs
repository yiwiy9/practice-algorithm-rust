use num_integer::gcd;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut dxy_set = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut dx = xy[j].0 - xy[i].0;
            let mut dy = xy[j].1 - xy[i].1;
            let d_gcd = gcd(dx.abs(), dy.abs());
            dx /= d_gcd;
            dy /= d_gcd;
            dxy_set.insert((dx, dy));
        }
    }

    println!("{}", dxy_set.len());
}
