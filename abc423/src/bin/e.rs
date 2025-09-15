use proconio::input;

/// https://atcoder.jp/contests/abc423/tasks/abc423_e
/// https://atcoder.jp/contests/abc423/editorial/13865
///
/// こういうときは、Aj に着目して、Aj が何回足されるかを考える。
/// Aj が足される回数の合計は Li ≤ l ≤ j ≤ r ≤ Ri を満たす整数組 (l,r) の数に等しい。
/// これは (j - Li + 1) * (Ri - j + 1) に等しい。
/// よって、答えは Σ Aj * (j - Li + 1) * (Ri - j + 1) となる。
/// 展開すると Σ -Aj * j^2 + (Li + Ri) * Aj * j + (-Li + 1) * (Ri + 1) * Aj となる。
/// これを高速に計算するには、Σ Aj * j^2, Σ Aj * j, Σ Aj の3つを前計算しておけば良い。
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let mut s_j = vec![0; n + 1];
    for i in 0..n {
        s_j[i + 1] = s_j[i] + a[i] * (i + 1) as i64;
    }

    let mut s_jj = vec![0; n + 1];
    for i in 0..n {
        s_jj[i + 1] = s_jj[i] + a[i] * ((i + 1) * (i + 1)) as i64;
    }

    for &(l, r) in &lr {
        println!(
            "{}",
            -(s_jj[r] - s_jj[l - 1])
                + (l + r) as i64 * (s_j[r] - s_j[l - 1])
                + (-(l as i64 - 1) * (r as i64 + 1)) * (s[r] - s[l - 1])
        );
    }
}
