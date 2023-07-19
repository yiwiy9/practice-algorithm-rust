use proconio::input;

/**
 * 3分探索
 * https://qiita.com/ganyariya/items/1553ff2bf8d6d7789127
 */
fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let f = |g: i64| a as f64 / (g as f64).sqrt() + b as f64 * (g as f64 - 1.0);

    let mut left = 1;
    let mut right = a / b; // g>=A/B のとき、f(g)>B*g≥A=f(1) より f(g)>f(1) であるため、1≤n<A/B の範囲だけ考えれば良い
    while right - left > 2 {
        // 1/3ずつ範囲を狭めていって極値を求める
        let mid_left = (left * 2 + right) / 3;
        let mid_right = (left + right * 2) / 3;

        if f(mid_left) > f(mid_right) {
            left = mid_left;
        } else {
            right = mid_right;
        }
    }

    let mut ans = a as f64;
    for g in left..=right {
        ans = ans.min(f(g));
    }
    println!("{:.8}", ans);
}
