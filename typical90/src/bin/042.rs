use proconio::input;

const MOD: usize = 1_000_000_007;

/**
 * https://twitter.com/e869120/status/1394423616805097477/photo/1
 *
 * (N mod 9) = (Σai*10^i mod 9) = Σ{(ai mod 9)*(10^i mod 9)} = Σ(ai mod 9)
 * => (N mod 9) = (Nの各桁の数字の和 mod 9)
 *
 * K が 9の倍数でないとき、答えは 0通り
 * K が 9の倍数であるとき、各位の数字の和が K である正整数の個数
 * => dp[各桁の数字の和] = 通り数
 */
fn main() {
    input! {
        k: usize,
    }

    if k % 9 != 0 {
        println!("{}", 0);
        return;
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;

    for i in 1..=k {
        let b = i.min(9);
        for j in 1..=b {
            dp[i] += dp[i - j];
            dp[i] %= MOD;
        }
    }

    println!("{}", dp[k]);
}
