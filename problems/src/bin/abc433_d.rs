use proconio::input;
use std::collections::HashMap;

/// https://atcoder.jp/contests/abc433/tasks/abc433_d
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   Aにある要素の 桁数とMで割った余り の個数を持てばいい
/// - それについて、何が分かれば答えになる？
///   Ai ごとに f(Ai, Aj) % M == 0 の個数を計算し、和を取れば答え
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   Aj の情報は桁数と割った余りのみで十分である。なぜなら f(Ai, Aj)の値が欲しいのではなく、 mod M == 0 の個数が欲しいのみだから
///   桁数は1~10なので十分小さいので、Aのループ中に全探索可能
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   Aの要素について、d_map[d] = HashMap(m, cnt) 桁数dである要素の内、余りがmである個数cntを前計算する
///   Aiでループし、その中でdでループし、 M - (Ai * 10^d) % M をキーとして d_map[d]の cntを取得し、ansに加える
///
/// AC: 30分
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut d_map = vec![HashMap::new(); 11];
    for &a_i in &a {
        let b = a_i;
        let d = num_digits(b, 10);
        *d_map[d].entry(a_i % m).or_insert(0) += 1_usize;
    }

    let mut ans: usize = 0;
    for &a_i in &a {
        for d in 1..=10 {
            let a_i_mod = (a_i * 10_i64.pow(d as u32) as usize) % m;
            ans += *d_map[d].get(&((m - a_i_mod) % m)).unwrap_or(&0);
        }
    }

    println!("{}", ans);
}

pub fn num_digits(mut n: usize, base: usize) -> usize {
    assert!(base >= 2);
    if n == 0 {
        return 1;
    }
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= base;
    }
    digits
}
