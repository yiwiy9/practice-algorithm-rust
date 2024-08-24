use itertools::Itertools as _;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n+1],
        mut c: [i32; n+m+1],
    }

    let mut b = vec![0; m + 1];

    // 【誤答】
    // for i in 0..m + 1 {
    //     let mut b_j = c[i];
    //     for k in 0..i {
    //         if i - k <= n && k <= m {
    //             b_j -= a[i - k] * b[k];
    //         }
    //     }
    //     // これだとb[i]がなんでも良くなるため絞れない
    //     if a[0] != 0 {
    //         b_j /= a[0];
    //     }
    //     b[i] = b_j;
    // }

    // 【正答】
    // https://atcoder.jp/contests/abc245/editorial/3651
    // a[0]ではなく、a[n](!=0)で割りたいので、逆順に解く
    for i in (0..m + 1).rev() {
        b[i] = c[i + n] / a[n];
        for j in 0..n + 1 {
            // dp的に決定したb[i]を使って、c[i+j]を更新していく
            c[i + j] -= a[j] * b[i];
        }
    }

    println!("{}", b.iter().join(" "));
}
