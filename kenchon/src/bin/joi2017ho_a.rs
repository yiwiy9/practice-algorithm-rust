use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: i64,
        t: i64,
        a: [i64; n+1],
        lrx: [(Usize1, Usize1, i64); q],
    }

    let mut a_diff = vec![0; n];
    for i in 0..n {
        a_diff[i] = a[i + 1] - a[i];
    }

    let mut cur = a_diff.iter().fold(0, |acc, &diff| {
        if diff > 0 {
            acc - diff * s
        } else {
            acc - diff * t
        }
    });

    for &(l, r, x) in &lrx {
        cur += if a_diff[l] > 0 {
            a_diff[l] * s
        } else {
            a_diff[l] * t
        };

        a_diff[l] += x;

        cur -= if a_diff[l] > 0 {
            a_diff[l] * s
        } else {
            a_diff[l] * t
        };

        if r < n - 1 {
            cur += if a_diff[r + 1] > 0 {
                a_diff[r + 1] * s
            } else {
                a_diff[r + 1] * t
            };

            a_diff[r + 1] -= x;

            cur -= if a_diff[r + 1] > 0 {
                a_diff[r + 1] * s
            } else {
                a_diff[r + 1] * t
            };
        }

        println!("{}", cur);
    }
}
