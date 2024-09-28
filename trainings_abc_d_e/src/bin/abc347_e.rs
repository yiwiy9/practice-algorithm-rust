use itertools::Itertools as _;
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }

    let mut set = BTreeSet::new();
    let mut s_size = vec![0; q];
    for (i, x_i) in x.iter().enumerate() {
        if set.contains(x_i) {
            set.remove(x_i);
        } else {
            set.insert(*x_i);
        }
        s_size[i] = set.len();
    }

    let mut s_size_sum = vec![0; q + 1];
    for i in 0..q {
        s_size_sum[i + 1] = s_size_sum[i] + s_size[i];
    }

    let mut n_idx = vec![vec![]; n + 1];
    for i in 0..q {
        n_idx[x[i]].push(i);
    }

    let mut a = vec![0; n + 1];
    for j in 1..=n {
        let mut is_plus = true;
        for &i in &n_idx[j] {
            let sum = s_size_sum[q] - s_size_sum[i];
            if is_plus {
                a[j] += sum;
            } else {
                a[j] -= sum;
            }
            is_plus = !is_plus;
        }
    }

    println!("{}", a.iter().skip(1).join(" "));
}
