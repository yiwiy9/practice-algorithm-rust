use proconio::input;
use superslice::*;

/// https://atcoder.jp/contests/joi2025yo2/tasks/joi2025_yo2_c
/// https://www2.ioi-jp.org/joi/2024/2025-yo2/2025-yo2-t3-review.pdf
///
/// 頻出の式変形: max(|x − a|, |x − b|) = |b−a| / 2 + |x − (a+b) / 2|
/// => 「2点 a,b から一番遠い距離」=「区間の半分 + 中点からの距離」
///
/// c は考察通り min/max のみを見るだけで良い
/// a はそういうわけにはいかないので全探索する必要がある
/// => b は二分探索で求めたい
///
/// max(|a + b + Cmin − P|, |a + b + Cmax − P|)
/// = max(|b - (P - a - Cmin)|, |b - (P - a - Cmax)|)
/// = |Cmax − Cmin| / 2 + |b − (P - a - (Cmin + Cmax) / 2)|
///
/// P - a - (Cmin + Cmax) / 2 との差が最小となる b を求める
fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        p: i64,
        a: [i64; x],
        mut b: [i64; y],
        c: [i64; z],
    }

    b.sort();
    let c_min = *c.iter().min().unwrap();
    let c_max = *c.iter().max().unwrap();

    let mut ans = 0;
    for &a_i in &a {
        let target = p - a_i - (c_min + c_max) / 2;
        let target_i = b.lower_bound(&target);

        let mut score = 1 << 60;
        if target_i < y {
            score = score.min(
                (a_i + b[target_i] + c_min - p)
                    .abs()
                    .max((a_i + b[target_i] + c_max - p).abs()),
            )
        }
        if target_i > 0 {
            score = score.min(
                (a_i + b[target_i - 1] + c_min - p)
                    .abs()
                    .max((a_i + b[target_i - 1] + c_max - p).abs()),
            )
        }

        ans = ans.max(score);
    }

    println!("{}", ans);
}
