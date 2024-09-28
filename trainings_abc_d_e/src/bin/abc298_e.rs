use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        p: usize,
        q: usize,
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for j in 0..=n {
        dp[n][j] = 1;
    }

    let p_inv = mod_inv(p, MOD);
    let q_inv = mod_inv(q, MOD);

    for i in (0..n).rev() {
        for j in (0..n).rev() {
            for i_p in 1..=p {
                for j_q in 1..=q {
                    let next_i = (i + i_p).min(n);
                    let next_j = (j + j_q).min(n);

                    dp[i][j] += dp[next_i][next_j] * p_inv % MOD * q_inv % MOD;
                    dp[i][j] %= MOD;
                }
            }
        }
    }

    println!("{}", dp[a][b]);
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
