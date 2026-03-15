use proconio::input;

/// https://atcoder.jp/contests/abc449/tasks/abc449_d
/// https://atcoder.jp/contests/abc449/editorial/17257
///
/// max(∣x∣,∣y∣) という式を簡潔に扱うために、
/// ∣x∣>∣y∣ であるものと
/// ∣x∣≤∣y∣ であるものに
/// 場合分けして数え上げ、最後に足し合わせることで答えを求めます
fn main() {
    input! {
        l: i64,
        r: i64,
        d: i64,
        u: i64,
    }
    let mut ans = 0;

    // |x| > |y|
    for x in l..=r {
        if x % 2 == 0 {
            // ∣x∣>∣y∣ は −∣x∣<y<∣x∣ と同値
            let cur_d = d.max(-x.abs() + 1);
            let cur_u = u.min(x.abs() - 1);
            let cnt = cur_u - cur_d + 1;
            ans += cnt.max(0);
        }
    }

    // |x| <= |y|
    for y in d..=u {
        if y % 2 == 0 {
            // ∣x∣<=∣y∣ は −∣y∣<=x<=∣y∣ と同値
            let cur_l = l.max(-y.abs());
            let cur_r = r.min(y.abs());
            let cnt = cur_r - cur_l + 1;
            ans += cnt.max(0);
        }
    }

    println!("{ans}")
}
