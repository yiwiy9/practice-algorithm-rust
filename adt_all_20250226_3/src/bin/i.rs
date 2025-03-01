use ac_library::FenwickTree;
use proconio::input;

const MOD: usize = 998244353;

/// https://atcoder.jp/contests/adt_all_20250226_3/tasks/abc276_f
/// https://atcoder.jp/contests/adt_all_20250226_3/editorial/5174
///
/// 差分を考える
/// kk{max(Ai,Aj)} = (k-1)(k-1){max(Ai,Aj)} + 2(k-1){max(Ai,Ak)} + Ak
/// {} は シグマ
///
/// あとは、(k-1){max(Ai,Ak)} を高速に求める
/// これは、1<=i<=k-1 の範囲のAiで、Akより大きいものはAiを選ぶ、小さいものはAkを選ぶということ
/// => Fenwick tree
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut sum = 0;
    let mut ft_cnt = FenwickTree::new(200_001, 0);
    let mut ft_sum = FenwickTree::new(200_001, 0);
    for (i, &a_i) in a.iter().enumerate() {
        let low_cnt = ft_cnt.sum(0..=a_i);
        let high_sum = ft_sum.sum(a_i + 1..200_001);
        let diff = (low_cnt * a_i + high_sum) % MOD;

        sum += 2 * diff + a_i;
        sum %= MOD;

        let ans = sum * mod_inv((i + 1) * (i + 1), MOD) % MOD;
        println!("{}", ans);

        ft_cnt.add(a_i, 1);
        ft_sum.add(a_i, a_i);
    }
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
pub fn mod_inv(num: usize, modulo: usize) -> usize {
    mod_pow(num, modulo - 2, modulo)
}
