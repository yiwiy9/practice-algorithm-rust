use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    let mut points = HashSet::new();
    for &(x, y) in &xy {
        points.insert((x, y));
    }

    let mut ans = 0;
    for i in 0..n {
        let (x1, y1) = xy[i];
        for j in i + 1..n {
            let (x2, y2) = xy[j];
            if x1 == x2 || y1 == y2 {
                continue;
            }
            if points.contains(&(x1, y2)) && points.contains(&(x2, y1)) {
                ans += 1;
            }
        }
        points.remove(&xy[i]);
    }

    println!("{}", ans);
}
