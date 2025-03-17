use proconio::input;

/// https://atcoder.jp/contests/abc397/tasks/abc397_d
/// https://atcoder.jp/contests/abc397/editorial/12454
/// x^3 − y^3 = N
///
/// x - y = d とすると、
/// N = x^3 - y^3 = (x-y)(x^2 + xy + y^2) = d(x^2 + xy + y^2) >= d(x^2 - 2xy + y^2) = d^3
/// よって、d <= N^(1/3) となり、全探索が可能
///
/// y = k とすると、
/// (k + d)^3 - k^3 = d^3 + 3d^2k + 3dk^2 = N
/// よって、d が与えられたときに、 3dk^2 + 3d^2k + d^3 - N = 0 という k に関する2次方程式を解けばよい
/// これは二次方程式の解の公式や二分探索を用いることで解ける
fn main() {
    input! {
        n: usize,
    }

    for d in 1..=n {
        let d3 = d * d * d;
        if d3 > n {
            break;
        }

        // 3dk^2 + 3d^2k + d^3 - N = 0

        // オーバーフロー対策
        // 3k^2 + 3dk + d^2 - N/d = 0
        if n % d != 0 {
            continue;
        }
        let n_div_d = n / d;

        let mut left = 0;
        let mut right = 1 << 30;
        while right - left > 1 {
            let mid = (left + right) / 2;

            if 3 * mid * mid + 3 * mid * d + d * d >= n_div_d {
                right = mid;
            } else {
                left = mid;
            }
        }

        if 3 * right * right + 3 * right * d + d * d == n_div_d {
            println!("{} {}", right + d, right);
            return;
        }
    }

    println!("-1");
}
