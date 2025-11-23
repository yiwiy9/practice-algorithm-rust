use proconio::{input, marker::Chars};
use std::collections::BTreeMap;

const MOD: usize = 998244353;

/// https://atcoder.jp/contests/abc433/tasks/abc433_f
/// https://atcoder.jp/contests/abc433/editorial/14594
/// => 高速化の手法が思いつかなかったら、式変形を試みる（特にシグマや二項係数は見えることがあるかもしれない）
/// => そのためにシグマで明示的にまとめるのもステップとして重要
/// => https://www.wolframalpha.com/ に式を入れると式変形してくれる
///
/// 畳み込み: 2つの関数から1つの関数を作る演算
/// 今回のテクニック: ヴァンデルモンドの畳み込み（https://manabitimes.jp/math/622）
/// (k=0-n)∑ pCk ⋅ qCn−k = p+qCn
fn main() {
    input! {
        s: Chars,
    }

    let mut left_cnt = vec![0; 10];
    let mut right_cnt = vec![0; 10];
    for &c in &s {
        right_cnt[c as usize - '0' as usize] += 1;
    }

    let mut map = BTreeMap::new();
    for &c in &s {
        let num = c as usize - '0' as usize;
        if num < 9 {
            map.entry((left_cnt[num], right_cnt[num + 1]))
                .and_modify(|cur| *cur += 1)
                .or_insert(1);
        }
        right_cnt[num] -= 1;
        left_cnt[num] += 1;
    }

    let comb = ModComb::new(s.len() + 1, MOD);

    let mut ans = 0;
    for ((left, right), &cnt) in &map {
        for _ in 0..cnt {
            ans += comb.combination(left + right, left + 1);
            ans %= MOD;
        }
    }

    println!("{}", ans);
}

pub struct ModComb {
    modulo: usize,
    fac: Vec<usize>,
    finv: Vec<usize>,
}
impl ModComb {
    pub fn new(cap: usize, modulo: usize) -> Self {
        let mut fac = vec![0; cap];
        let mut finv = vec![0; cap];
        let mut inv = vec![0; cap];
        fac[0] = 1;
        fac[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;
        for i in 2..cap {
            fac[i] = fac[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            finv[i] = finv[i - 1] * inv[i] % modulo;
        }
        Self { modulo, fac, finv }
    }
    pub fn combination(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        self.fac[n] * (self.finv[k] * self.finv[n - k] % self.modulo) % self.modulo
    }
    pub fn homogeneous(&self, n: usize, k: usize) -> usize {
        self.combination(n + k - 1, k)
    }
    pub fn large_n_combination(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        let mut res = 1;
        for i in (n - k + 1)..=n {
            res = res * i % self.modulo;
        }
        res * self.finv[k] % self.modulo
    }
}
