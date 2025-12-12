use proconio::input;
const MOD: usize = 998244353;

fn main() {
    input! {
        k: usize,
        c: [usize; 26],
    }

    let comb = ModComb::new(10000, MOD);

    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for i in 0..26 {
        let mut next_dp = dp.clone();
        for j in 0..=k {
            if dp[j] == 0 {
                continue;
            }

            for cnt in 1..=c[i] {
                if j + cnt > k {
                    continue;
                }
                let next_j = j + cnt;
                let next_value = comb.combination(next_j, cnt);
                next_dp[next_j] += (next_value * dp[j]) % MOD;
                next_dp[next_j] %= MOD;
            }
        }
        dp = next_dp;
    }

    println!("{}", dp.iter().skip(1).fold(0, |acc, x| (acc + x) % MOD));
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
