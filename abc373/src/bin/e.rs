use itertools::Itertools as _;
use proconio::input;
use superslice::*;

/**
 * https://atcoder.jp/contests/abc373/tasks/abc373_e
 * https://atcoder.jp/contests/abc373/editorial/11044
 *
 * 最初に上位の和を取ってそれをMで割って判定してたのがあかん理由
 * => すでに当確の人の票を他の人に分配してしまってることになるから
 * => 累積和が必要
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        a: [i64; n],
    }

    if n == m {
        println!("{}", a.iter().map(|_| 0).join(" "));
        return;
    }

    let mut b = a.clone();
    b.sort();

    let mut b_s = vec![0; n + 1];
    for i in 0..n {
        b_s[i + 1] = b_s[i] + b[i];
    }

    let rest = k - a.iter().sum::<i64>();
    let mut ans = vec![0; n];
    for (i, &a_i) in a.iter().enumerate() {
        let a_i_idx = b.lower_bound(&(a_i));

        let mut left = -1;
        let mut right = rest + 1;
        while right - left > 1 {
            let mid = (left + right) / 2;

            let cur_a_i = a_i + mid;

            let ok_min_idx = b.upper_bound(&cur_a_i);
            let ok_num = n - ok_min_idx;
            let rest_num = (m as i64 - ok_num as i64).max(0) as usize;

            let ng_sum = if ok_min_idx - (a_i_idx + 1) >= rest_num {
                b_s[ok_min_idx] - b_s[ok_min_idx - rest_num]
            } else {
                b_s[ok_min_idx] - b_s[a_i_idx + 1] + b_s[a_i_idx]
                    - b_s[a_i_idx - (rest_num - (ok_min_idx - (a_i_idx + 1)))]
            };

            if (cur_a_i + 1) * rest_num as i64 - ng_sum <= rest - mid {
                left = mid;
            } else {
                right = mid;
            }
        }

        ans[i] = if right <= rest { right } else { -1 }
    }

    println!("{}", ans.iter().join(" "));
}
