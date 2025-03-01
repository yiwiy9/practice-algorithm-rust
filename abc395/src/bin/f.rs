use proconio::input;

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        x: i64,
        ud: [(i64,i64); n],
    }

    let mut left = 0;
    let mut right = INF;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut ok = true;
        let mut lh = vec![(0, 0); n];
        for i in 0..n {
            let (u, d) = ud[i];
            if u + d < mid {
                ok = false;
                break;
            }

            let low = (mid - d).max(0);
            let high = u;

            if i == 0 {
                lh[i] = (low, high);
            } else {
                let (pl, ph) = lh[i - 1];
                let cl = (pl - x).max(0);
                let ch = ph + x;
                lh[i] = (low.max(cl), high.min(ch));

                if lh[i].0 > lh[i].1 {
                    ok = false;
                    break;
                }
            }
        }

        if ok {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", ud.iter().map(|(u, d)| u + d - left).sum::<i64>());
}
