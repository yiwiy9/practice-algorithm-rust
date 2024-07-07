use proconio::input;

const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc356/tasks/abc356_d
 * https://atcoder.jp/contests/abc356/editorial/10126
 *
 * j=0,1,…,59 について、以下を繰り返す。
 * - M の j bit 目が立っていなければ何もしない。
 * - M の j bit 目が立っていれば、 0 以上 N 以下の整数のうち j bit 目が立っているものの個数を答えに加算する。
 *
 * 後は以下の数え上げが出来ればこの問題に正解できる
 * → 0 以上 N 以下の整数のうち j bit 目が立っているものの個数を求める
 */
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = 0;
    for j in 0..60 {
        if (m & (1 << j)) != 0 {
            ans += calc(j, n);
            ans %= MOD;
        }
    }

    println!("{}", ans % MOD);
}

fn calc(j: usize, n: usize) -> usize {
    let j_2 = 1 << j;
    let k = n / (2 * j_2);

    let mut res = k * j_2; // nの商の個数
    let l = n % (2 * j_2); // nの余り
    if l >= j_2 {
        // j_2以下の数字でj bit目が立っているものの個数は j_2
        res += l - j_2 + 1;
    }

    res
}
