use proconio::input;
const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
    }

    // n要素それぞれが1~kの値をとる配列 => k^n通り
    // 上記の配列それぞれに1~m点数を割り当てる => m^(k^n)通り
    // フェルマーの小定理より、M^(k^n) = M^(q(P-1) + r) = M^r (mod P)

    // mがMODの倍数なら、m^(k^n)もMODの倍数
    if m % MOD == 0 {
        println!("0");
        return;
    }

    let k_n = mod_pow(k, n, MOD - 1);
    let m_k_n = mod_pow(m, k_n, MOD);

    println!("{}", m_k_n);
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
