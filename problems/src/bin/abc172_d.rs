use proconio::input;

/**
 * https://atcoder.jp/contests/abc172/tasks/abc172_d
 * 正整数 X に対し、X の正の約数の個数を f(X) とします。
 * 正整数 N が与えられるので、∑ K×f(K) を求めてください。
 *
 * 愚直に計算すると、O(N√N)かかるので工夫が必要
 * => 約数の個数の表を作る O(N logN)
 *    https://maspypy.com/atcoder-%e5%8f%82%e5%8a%a0%e6%84%9f%e6%83%b3-2020-06-27abc-172#toc4
 */
fn main() {
    input! {
        n: usize,
    }

    // iを約数に持つn以下の数のカウント(*自身の数字)
    let mut ans = 0;
    for i in 1..=n {
        for a in 1..=n {
            if i * a > n {
                break;
            }
            ans += i * a;
        }
    }

    println!("{}", ans)
}
