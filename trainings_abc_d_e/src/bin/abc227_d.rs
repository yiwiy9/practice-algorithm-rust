use proconio::input;
const INF: usize = 1 << 60;

/**
 * https://atcoder.jp/contests/abc227/tasks/abc227_d
 * https://atcoder.jp/contests/abc227/editorial/2911
 * 青diffだと怖気づくな！
 * 決め打ち二分探索で解ける！！
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut left = 0;
    let mut right = INF / k; // オーバーフロー対策
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut sum = 0;
        for &a_i in &a {
            sum += a_i.min(mid);
        }

        if mid * k <= sum {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
