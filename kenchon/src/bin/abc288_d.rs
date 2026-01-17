use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc288/tasks/abc288_d
/// https://atcoder.jp/contests/abc288/editorial/5664
/// 不変量: 操作などの変換によって変化しない性質
/// を見つけると、スムーズに解ける問題
///
/// まず、i mod K=j なる Xi の総和を Sj とします。
/// このとき、連続する K 個の要素にたいして c の加算を行う時、S0, S1,…, SK−1 それぞれ c だけ変化することがわかります。
/// すなわち、各 j (1≤i≤K−1) にたいして、S0 − Sj は変化しません。
/// よってこの量が不変量となります。
///
/// => この問題は各クエリに対して S0 = S1 = … = SK−1 であるかを判定できればよい
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        q: usize,
        lr: [(Usize1,Usize1); q],
    }

    let mut s = vec![vec![0; n + 1]; k];
    for j in 0..k {
        for i in 0..n {
            s[j][i + 1] = s[j][i] + if i % k == j { a[i] } else { 0 };
        }
    }

    for &(l, r) in &lr {
        let mut ok = true;
        let s_0 = s[0][r + 1] - s[0][l];
        for j in 0..k {
            ok &= s_0 == (s[j][r + 1] - s[j][l]);
        }
        println!("{}", if ok { "Yes" } else { "No" });
    }
}
