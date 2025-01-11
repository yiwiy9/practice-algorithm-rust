use proconio::input;

/// https://atcoder.jp/contests/adt_all_20240314_1/tasks/abc254_c
/// https://atcoder.jp/contests/adt_all_20240314_1/editorial/4051
/// 問題をちゃんと考察する
/// k == 1 の場合は、バブルソートになるので Yes
/// k >= 2 の場合は、ai, ai+k, ai+2k, ... は昇順にすることができる
/// あとは、繋げたものが昇順になっているかどうかを確認する
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    let mut b = vec![vec![]; k];
    for i in 0..n {
        b[i % k].push(a[i]);
    }

    b.iter_mut().for_each(|v| v.sort());

    let mut c = vec![];
    for i in 0..n {
        c.push(b[i % k][i / k]);
    }

    a.sort();
    println!("{}", if a == c { "Yes" } else { "No" });
}
