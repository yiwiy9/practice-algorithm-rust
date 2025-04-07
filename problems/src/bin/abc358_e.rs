use ac_library::modint::ModInt998244353 as Mint;
use proconio::input;

const MOD: usize = 998244353;

/// https://atcoder.jp/contests/abc358/tasks/abc358_e
/// https://atcoder.jp/contests/abc358/editorial/10224
///
/// 1桁ずつではなく、1文字ずつ決めていく
/// dp[i][j] := a1, a2, ..., aiからなる長さjの文字列であって、
///             各1<=k<=iにたいしてakが0個以上Ck個以下であるものの個数
///
/// 1文字ずつ決める際に、長さsの文字列中にaiがt個存在するとすると、位置の決め方は sCk
fn main() {
    input! {
        k: usize,
        c: [usize; 26],
    }

    let comb = ModComb::new(1_001, MOD);
    let mut dp = vec![vec![Mint::new(0); k + 1]; 27];
    dp[0][0] = Mint::new(1);

    for i in 0..26 {
        for j in 0..=k {
            let mut sum = Mint::new(0);
            for t in 0..=c[i].min(j) {
                sum += dp[i][j - t] * comb.combination(j, t);
            }
            dp[i + 1][j] = sum;
        }
    }

    println!("{}", dp[26].iter().skip(1).sum::<Mint>());
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
