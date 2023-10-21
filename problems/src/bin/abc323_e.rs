use proconio::input;

const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc323/tasks/abc323_e
 * https://atcoder.jp/contests/abc323/editorial/7357
 *
 * 確率DP
 * 各非負整数t=0,1,…,Xについて、時刻tに曲が切り替わる(新しく曲が選ばれ再生され始める)確率をp[t]とする
 * p[0]=1
 * p[t]は、k=1,2,…,Nについて、「ちょうど時刻t−Tkにk番目の曲が再生され始めた確率」の和
 *
 * => 各t=X,(X−1),…,(X−T1+1) について、「時刻tに1番目の曲が再生され始めた確率」を足し合わせれば良い
 */
fn main() {
    input! {
        n: usize,
        x: usize,
        t: [usize; n],
    }

    let mod_inv_n = mod_inv(n, MOD);

    let mut p = vec![0; x + 1];
    p[0] = 1;
    for i in 1..=x {
        for j in 0..n {
            if t[j] <= i {
                p[i] += p[i - t[j]];
                p[i] %= MOD;
            }
        }
        p[i] *= mod_inv_n;
        p[i] %= MOD;
    }

    let mut ans = 0;
    for i in 0..t[0] {
        if x >= i {
            ans += p[x - i] * mod_inv_n; // 曲1を選ぶ
            ans %= MOD;
        }
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
