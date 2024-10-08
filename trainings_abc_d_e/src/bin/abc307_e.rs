use proconio::input;

const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc307/tasks/abc307_e
 * https://atcoder.jp/contests/abc307/editorial/6643
 *
 * 円環上のDP
 * => 状態がループしている問題をDPで解く場合、1箇所を決め打ってDPを進め、
 *    1周した最後に、末尾と先頭の条件が整合的であるもののみを考慮して解くことができる
 *
 * => ただそのままやると計算量が莫大なので、状態を減らす工夫が必要
 * => 直前の人に何の数を割り当てた数を全て区別する必要はなく、
 *   「1人目に割り当てた数と同じであるか」の情報さえあれば答えを求められることがわかる
 *
 * => DP[i][f]= i人目まで数を割り当てる方法のうち、
 *              i人目に割り当てた数が1人目に割り当てた数と同じで (f ? ある:ない) ものの個数
 */
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut dp = vec![vec![0; 2]; n + 1];
    dp[1][1] = m;
    for i in 1..n {
        dp[i + 1][0] += dp[i][0] * (m - 2); // 1人目とi人目以外のm-2通り
        dp[i + 1][0] += dp[i][1] * (m - 1); // 1人目(=i人目)以外のm-1通り
        dp[i + 1][0] %= MOD;

        dp[i + 1][1] += dp[i][0]; // 1人目とi人目が同じ数の通り
        dp[i + 1][1] %= MOD;
    }

    println!("{}", dp[n][0]);
}
