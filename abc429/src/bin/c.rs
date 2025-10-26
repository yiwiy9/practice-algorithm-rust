use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut left = vec![0; n];
    let mut right = vec![0; n];
    for &a_i in &a {
        right[a_i] += 1;
    }

    left[a[0]] += 1;
    right[a[0]] -= 1;

    let mut same = left[a[0]] * right[a[0]];
    let mut ans = 0;
    for j in 1..n - 1 {
        same -= left[a[j]];
        right[a[j]] -= 1;

        let left_diff_cnt = j - left[a[j]];
        let right_diff_cnt = n - (j + 1) - right[a[j]];

        ans += left_diff_cnt * right[a[j]];
        ans += right_diff_cnt * left[a[j]];
        ans += same - left[a[j]] * right[a[j]];

        left[a[j]] += 1;
        same += right[a[j]];
    }

    println!("{}", ans);
}
