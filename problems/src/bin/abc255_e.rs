use proconio::input;
use std::collections::HashMap;

/// https://atcoder.jp/contests/abc255/tasks/abc255_e
/// https://atcoder.jp/contests/abc255/editorial/4098
/// Z = A[0] を一つ決めれば、残りのA[i] は決まる。
/// A[1] = S[0] - Z
/// A[2] = S[1] - S[0] + Z
/// A[3] = S[2] - S[1] + S[0] - Z
/// ...
/// B[i] = S[i-1] - B[i-1] (B[0] = 0)とすると、
/// A[i] = (-1)^i * Z + B[i] となる。
/// これが、 = X[j] となるような個数が最大の Z を求める。
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [i64; n-1],
        x: [i64; m],
    }

    let mut b = vec![0; n];
    for i in 1..n {
        b[i] = s[i - 1] - b[i - 1];
    }

    let mut map = HashMap::new();
    for i in 0..n {
        for &x_j in &x {
            // z = (-1)^i * (x_j - b_i)
            let z = if i % 2 == 0 { x_j - b[i] } else { -x_j + b[i] };
            *map.entry(z).or_insert(0) += 1;
        }
    }

    println!("{}", map.values().max().unwrap_or(&0));
}
