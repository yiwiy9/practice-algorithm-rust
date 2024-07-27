use proconio::input;
use superslice::*;

const INF: isize = 1 << 60;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [isize; n],
        bk: [(isize,usize); q],
    }

    a.sort();
    let b: Vec<isize> = bk.iter().map(|x| x.0).collect();

    for i in 0..q {
        let mut left = -1;
        let mut right = INF;
        while right - left > 1 {
            let mid = (left + right) / 2;

            let a_plus = b[i] + mid;
            let a_plus_idx = a.upper_bound(&a_plus) - 1;

            let a_minus = b[i] - mid;
            let a_minus_idx = a.lower_bound(&a_minus);

            let kth = a_plus_idx + 1 - a_minus_idx;

            if kth >= bk[i].1 {
                right = mid;
            } else {
                left = mid;
            }
        }
        println!("{}", right);
    }
}
