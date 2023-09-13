use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let comb = ModComb::new(200_001, MOD);

    let mut ans = mod_pow(2, n, MOD);
    ans += MOD - 1;
    ans %= MOD;
    ans += MOD - comb.large_n_combination(n, a);
    ans %= MOD;
    ans += MOD - comb.large_n_combination(n, b);
    ans %= MOD;

    println!("{}", ans);
}

pub fn mod_pow(base: usize, exp: usize, modulo: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    let mut result = mod_pow(base * base % modulo, exp / 2, modulo);
    if exp % 2 == 1 {
        result *= base;
        result %= modulo;
    }
    result
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
