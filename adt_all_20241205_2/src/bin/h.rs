use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut right_num_cnt = vec![0; n];
    for i in 0..n {
        right_num_cnt[a[i]] += 1;
    }

    let mut left_num_cnt = vec![0; n];
    let mut ans = 0_usize;
    let mut cur = 0;
    for i in 0..n {
        cur -= right_num_cnt[a[i]] * left_num_cnt[a[i]];

        ans += cur;

        right_num_cnt[a[i]] -= 1;
        left_num_cnt[a[i]] += 1;
        cur += right_num_cnt[a[i]] * left_num_cnt[a[i]];
    }

    println!("{}", ans);
}
