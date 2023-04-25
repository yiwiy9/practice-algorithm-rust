use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    let mod_inv_2 = mod_inv(2, MOD);

    input! {
        l: usize,
        r: usize,
    }

    let mut ans: usize = 0;
    let mut x: usize = 1;
    for d in 1..=19 {
        if l >= x * 10 {
            x *= 10;
            continue;
        }
        if r < x {
            break;
        }
        let l_d = l.max(x) % MOD;
        let r_d = r.min(x * 10 - 1) % MOD;

        // 和の公式： n*(a+l)/2
        let o = (r_d + MOD - l_d + 1) % MOD; // usize: 一瞬でも負にならないようにMODを足す
        let p = (l_d + r_d) * mod_inv_2 % MOD;
        let q = (o * p % MOD) * d % MOD;
        ans += q;
        ans %= MOD;

        x *= 10;
    }

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
pub fn mod_inv(num: usize, modulo: usize) -> usize {
    mod_pow(num, modulo - 2, modulo)
}
