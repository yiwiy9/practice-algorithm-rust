use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut ans = 0;
    let mut left = 0;
    let mut right = 1_000_000_000 + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut less_cnt: i64 = 0;
        let mut less_sum: i64 = 0;
        let mut more_cnt: i64 = 0;
        let mut more_sum: i64 = 0;

        for &a_i in &a {
            if a_i <= mid {
                less_cnt += 1;
                less_sum += mid - a_i;
            } else {
                more_cnt += 1;
                more_sum += a_i - (mid + 1);
            }
        }

        if less_sum >= more_sum {
            if less_sum - more_sum <= more_cnt {
                left = mid;
                ans = less_sum;
            } else {
                right = mid;
            }
        } else {
            if more_sum - less_sum <= less_cnt {
                right = mid;
                ans = more_sum;
            } else {
                left = mid;
            }
        }
    }

    println!("{}", ans);
}
