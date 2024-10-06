use proconio::input;

/**
 * https://atcoder.jp/contests/abc374/tasks/abc374_e
 * https://atcoder.jp/contests/abc374/editorial/11094
 *
 * 実は、以下の事柄が成り立ちます。
 * > 工程 i を w 個以上処理できる機械の導入方法のうち最小の金額を達成するものでは、次の少なくとも一方が満たされる。
 * > - 機械 Si が Bi 台以下しか導入されない。
 * > - 機械 Ti が Ai 台以下しか導入されない。
 *
 * => なぜなら、両方AiBi個分の処理能力を持つので、上記以上の場合はコストが安い方に交換すれば良いから。
 */
fn main() {
    input! {
        n: usize,
        x: usize,
        apbq: [(usize,usize,usize,usize); n],
    }

    let mut left = 0;
    let mut right = 1_000_000_000 + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut sum_cost = 0;
        for &(a, p, b, q) in &apbq {
            let mut min_cost = 10_000_000 + 1;

            for s in 0..=b {
                let mut cur = s * p;
                if s * a < mid {
                    let t = (mid - s * a) / b + if (mid - s * a) % b > 0 { 1 } else { 0 };
                    cur += t * q;
                }
                min_cost = min_cost.min(cur);
            }

            for t in 0..=a {
                let mut cur = t * q;
                if t * b < mid {
                    let s = (mid - t * b) / a + if (mid - t * b) % a > 0 { 1 } else { 0 };
                    cur += s * p;
                }
                min_cost = min_cost.min(cur);
            }

            sum_cost += min_cost;
        }

        if sum_cost <= x {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
