use proconio::input;

const INF: i64 = 1 << 30;

/// https://atcoder.jp/contests/abc374/tasks/abc374_e
/// https://atcoder.jp/contests/abc374/editorial/11094
/// 最適解において、機械 Si,Ti のどちらかについて、AiBi 個分以下の処理能力しか持たせなくて良い。
fn main() {
    input! {
        n: usize,
        x: i64,
        apbq: [(i64,i64,i64,i64); n],
    }

    let mut left = -1;
    let mut right = INF;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut y = 0;
        for &(a, p, b, q) in &apbq {
            let mut min_y = INF;

            // Tiの方が処理能力が高い場合、Siの個数はBi個以下で良い
            for a_cnt in 0..=b {
                let mut cur_y = a_cnt * p;
                let b_w = (mid - (a_cnt * a)).max(0);
                let b_cnt = (b_w + b - 1) / b;
                cur_y += b_cnt * q;
                min_y = min_y.min(cur_y);
            }
            // Siの方が処理能力が高い場合、Tiの個数はAi個以下で良い
            for b_cnt in 0..=a {
                let mut cur_y = b_cnt * q;
                let a_w = (mid - (b_cnt * b)).max(0);
                let a_cnt = (a_w + a - 1) / a;
                cur_y += a_cnt * p;
                min_y = min_y.min(cur_y);
            }

            y += min_y;
            if y > x {
                break;
            }
        }

        if y <= x {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
