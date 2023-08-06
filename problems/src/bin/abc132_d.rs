use proconio::input;
const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let comb = ModComb::new(n + 1, MOD);
    for i in 1..=k {
        let blue_pos_comb = comb.combination(n - k + 1, i);

        let left_blue = k - i;
        let left_blue_pos_comb = comb.homogeneous(i, left_blue);

        println!("{}", blue_pos_comb * left_blue_pos_comb % MOD);
    }
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
}
