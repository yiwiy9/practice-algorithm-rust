use proconio::input;

const MOD: usize = 998244353;

/// https://atcoder.jp/contests/abc431/tasks/abc431_f
/// https://atcoder.jp/contests/abc431/editorial/14492
///
/// 条件: すべての i (1≤i≤N−1) に対して Bi+1 ≥ Bi − D が成り立つ。
/// => B において，ある要素 v の直前に v+D より大きい要素は置くことができない
///
/// この考えをもとにして，値が小さい方から B の相対的な位置を決めていく 挿入dp でこの問題を解きます．
///
/// v 未満の相対的な位置が決まっていたとします．
/// このとき，v を挿入できる位置は，（現在の B に依らず）v−D 以上 v 未満の数の個数に依存しています．
/// => 挿入できる箇所を数式で書くと cnt[v−D] +… +cnt[v−1] +1 箇所です．（+1 は末尾に追加できることを表しています.）
///
/// 挿入 dp (挿入dp)
/// dp[i] = ( 列 B の先頭 i 項の並べ替え後の前後関係を決定した状態に関する小問題の解 )
/// ・値の小さい方から B が決められるのがミソ.
/// ・順列を前から作っていくのではなく、遷移の際に「1,2,…,i−1 からなる順列のどこに i を挿入するか」を考える.
fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    let a_max = *a.iter().max().unwrap();

    let mut cnt = vec![0; a_max + 1];
    for &a_i in &a {
        cnt[a_i] += 1;
    }

    let comb = ModComb::new(n + 1, MOD);

    let mut ans = 1;
    let mut sum = 0;
    for v in 0..=a_max {
        sum += cnt[v];

        ans *= comb.combination(sum, cnt[v]);
        ans %= MOD;

        if v >= d {
            sum -= cnt[v - d];
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
