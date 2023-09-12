use proconio::input;

const MOD: usize = 1_000_000_007;

/**
 * https://atcoder.jp/contests/abc145/tasks/abc145_d
 * 1 回の移動で x 座標 +y 座標の値は 3 増える
 * なので X+Y が 3 の倍数でないとき答えは 0
 *
 * n + 2m = X, 2n + m = Y という連立方程式が得られ、n < 0 または m < 0 のとき答えは 0
 * そうでないとき、計 n + m 回の移動のうち、どの n 回で (+1,+2) の移動をするか決めればよいので、答えは n+mCn
 */
fn main() {
    input! {
        mut x: i64,
        mut y: i64,
    }

    if (x + y) % 3 != 0 {
        println!("{}", 0);
        return;
    }

    let n_3 = 2 * y - x;
    let m_3 = x + y - n_3;
    if n_3 < 0 || m_3 < 0 {
        println!("{}", 0);
        return;
    }

    let n = (n_3 / 3) as usize;
    let m = (m_3 / 3) as usize;

    let comb = ModComb::new(n + m + 1, MOD);
    println!("{}", comb.combination(n + m, n));
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
