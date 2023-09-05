use proconio::input;

const MOD: usize = 1_000_000_007;

/**
 * https://atcoder.jp/contests/abc163/tasks/abc163_d
 *
 * 【気づくべき性質】
 * M 個の数を選ぶ場合、和としてあり得る最小値は、小さい方から M 個とった場合であり、最大値は大きい方から M 個取った場合です。
 * 実は最小値と最大値の間の値は全て作ることができます。
 * （最小値から始めて、選ぶ数を少しずつ大きくしていくことを想像するとわかります）
 */
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut s = vec![0; n + 2];
    for i in 0..=n {
        s[i + 1] = s[i] + i;
    }

    let mut ans = 0;
    for j in k..=n + 1 {
        let max_sum = s[n + 1] - s[n + 1 - j];
        let min_sum = s[j] - s[0];
        let range_sum = (max_sum - min_sum + 1) % MOD;
        ans += range_sum;
        ans %= MOD;
    }

    println!("{}", ans);
}
