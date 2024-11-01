use itertools::Itertools;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc377/tasks/abc377_e
/// https://atcoder.jp/contests/abc377/editorial/11238
/// https://x.com/kyopro_friends/status/1851568404282699973
///
/// iをP[i]に置き換えると『1個進む』
/// 数列を書き換えるのではなく、新しい数列を作ると考える
/// => P0[i] = P[i], P1[i] = P0[P0[i]], P2[i] = P1[P1[i]], ...
///
/// i -> P0[i] -> P0[P0[i]]
///                  ||
/// i    (2)->      P1[i]    (2)->    P1[P1[i]]]
///                                      ||
/// i               (4)->               P2[i]    (4)->    P2[P2[i]]
///
/// このように考えると、K回進んだ後の数列は P^(2^K) で表せる
///
/// あとは、それぞれのサイクルについて、周期性を利用して、K回進んだ後の数列を求める
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    }

    let mut ans = vec![0; n];
    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut cycle = vec![];
        let mut cur = i;
        while !seen[cur] {
            seen[cur] = true;
            cycle.push(cur);
            cur = p[cur];
        }

        let cycle_len = cycle.len();
        let cycle_mod = mod_pow(2, k, cycle_len);
        for j in 0..cycle_len {
            ans[cycle[j]] = cycle[(j + cycle_mod) % cycle_len];
        }
    }

    println!("{}", ans.iter().map(|&x| x + 1).join(" "));
}

pub fn mod_pow(base: usize, exp: usize, modulo: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    let base = base % modulo;
    let mut result = mod_pow(base * base % modulo, exp / 2, modulo);
    if exp % 2 == 1 {
        result *= base;
        result %= modulo;
    }
    result
}
