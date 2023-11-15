use proconio::input;

/**
 * https://atcoder.jp/contests/abc148/tasks/abc148_e
 * https://blog.hamayanhamayan.com/entry/2019/12/23/220145
 *
 * 整数 N が与えられます。f(N) を 10 進法で表記した時に末尾に何個の 0 が続くかを求めてください。
 */
fn main() {
    input! {
        mut n: usize,
    }

    if n % 2 != 0 {
        println!("0");
        return;
    }

    // 10の倍数の個数
    let mut ans = n / 10;

    // 10の位以上の数が5の倍数の個数（eg. 50, 250, ...）
    n /= 10;
    let mut div = 5;
    while n / div > 0 {
        ans += n / div;
        div *= 5;
    }

    println!("{}", ans);
}
