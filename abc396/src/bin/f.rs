use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::input;

/// https://atcoder.jp/contests/abc396/tasks/abc396_f
/// https://atcoder.jp/contests/abc396/editorial/12374
///
/// 1. Ai < Aj のとき
///     Bi > Bj となる k の集合は [M − Aj, M − Ai) です。
/// 2. Ai = Aj のとき
///     Bi > Bj となる k は存在しません。
/// 3. Ai > Aj のとき
///     Bi > Bj となる k の集合は [0, M − Ai) と [M − Aj, M) です。
///
/// これらの事実とimos法を用いることで解くことができます。
/// ただし、i, j のそれぞれでループを回すと、O(N^2) になってしまいます。
/// そこで、カウンタとなる配列を用意して j の個数を順に数えていくことで、O(N) で解くことができます。
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    //まず k = 0 の場合の転倒数を FenwickTree で求める
    let mut inversion = 0_i64;
    let mut ft = FenwickTree::new(m, 0);
    for &ai in &a {
        inversion += ft.sum(ai + 1..m);
        ft.add(ai, 1);
    }

    let mut imos = vec![0; m + 1];
    // 3 の 0 に +1 する, M に -1 する部分を imos に追加
    imos[0] += inversion;
    imos[m] -= inversion;

    let mut num_cnt = vec![0; m + 1];
    for (j, &a_j) in a.iter().enumerate() {
        // 1, 3 の M − Aj に +1 する部分を imos に追加
        // i < j かつ Ai != Aj のときと条件をまとめて考えられるので、
        // j から それまでに出てきた Aj の数を引いていく
        imos[m - a_j] += j as i64 - num_cnt[a_j];
        num_cnt[a_j] += 1;
    }

    let mut num_cnt = vec![0; m + 1];
    for (j, &a_j) in a.iter().rev().enumerate() {
        // 1, 3 の M − Aj に -1 する部分を imos に追加
        // i > j かつ Ai != Aj のときと条件をまとめて考えられるので、
        // それ以降に出てくる Aj の数を足していく
        imos[m - a_j] -= j as i64 - num_cnt[a_j];
        num_cnt[a_j] += 1;
    }

    // imos を累積和に変換
    for i in 1..=m {
        imos[i] += imos[i - 1];
    }

    println!("{}", imos.iter().take(m).join("\n"));
}
