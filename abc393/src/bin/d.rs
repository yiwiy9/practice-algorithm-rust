use proconio::{input, marker::Chars};

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut left_cnt = vec![0; n];
    let mut one_cnt = 0;
    for (i, &c) in s.iter().enumerate() {
        if c == '1' {
            one_cnt += 1;
        } else {
            left_cnt[i] = one_cnt;
        }
    }

    let mut right_cnt = vec![0; n];
    let mut one_cnt = 0;
    for (i, &c) in s.iter().enumerate().rev() {
        if c == '1' {
            one_cnt += 1;
        } else {
            right_cnt[i] = one_cnt;
        }
    }

    let mut left_cnt_sum = vec![0; n + 1];
    for i in 0..n {
        left_cnt_sum[i + 1] = left_cnt_sum[i] + left_cnt[i];
    }
    let mut right_cnt_sum = vec![0; n + 1];
    for i in 0..n {
        right_cnt_sum[i + 1] = right_cnt_sum[i] + right_cnt[i];
    }

    let mut ans = INF;
    for i in 1..=n {
        ans = ans.min(left_cnt_sum[i] + right_cnt_sum[n] - right_cnt_sum[i]);
    }

    println!("{}", ans);
}
