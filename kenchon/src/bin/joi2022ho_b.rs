use itertools::Itertools;
use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut a_i_sorted = a
        .iter()
        .enumerate()
        .map(|(i, &a_i)| (a_i.max(b[i]), i))
        .collect_vec();
    a_i_sorted.sort_by(|a, b| b.cmp(a));

    let mut left = 0;
    let mut right = INF;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut ok = true;
        let mut available = 0;
        for &(a_i, i) in &a_i_sorted {
            let a_days = mid.div_ceil(a_i);
            if a_days <= m {
                available += m - a_days;
                continue;
            }

            let diff = mid - (a_i * m);
            let b_days = diff.div_ceil(b[i]);
            if b_days <= available {
                available -= b_days;
                continue;
            }

            ok = false;
            break;
        }

        if ok {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
