use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64,i64); n],
    }

    let mut ans = 0;
    for comb in xy.iter().combinations(3) {
        let (a, b, c) = (comb[0], comb[1], comb[2]);
        let (x1, y1) = a;
        let (x2, y2) = b;
        let (x3, y3) = c;

        let a = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);
        if a != 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
