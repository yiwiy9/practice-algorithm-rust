use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    // 今i回ルーレットを回していて、マスjにいるときにゴールできる確率
    let mut dp = vec![vec![0; n + 1]; k + 1];

    for i in 0..=k {
        dp[i][n] = 1;
    }

    let m_inv = mod_inv(m, MOD);

    for i in (0..k).rev() {
        for j in 0..n {
            for r in 1..=m {
                if j + r <= n {
                    dp[i][j] += dp[i + 1][j + r] * m_inv;
                    dp[i][j] %= MOD;
                } else {
                    dp[i][j] += dp[i + 1][n - (j + r - n)] * m_inv;
                    dp[i][j] %= MOD;
                }
            }
        }
    }

    println!("{}", dp[0][0]);
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
