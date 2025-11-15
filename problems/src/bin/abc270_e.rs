use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut left = 0;
    let mut right: usize = 1 << 60;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut cnt = 0;
        for &a_i in &a {
            cnt += a_i.min(mid);
        }

        if cnt < k {
            left = mid;
        } else {
            right = mid;
        }
    }

    let mut cnt = 0;
    let mut ans = vec![0; n];
    for i in 0..n {
        let sub = a[i].min(left);
        cnt += sub;
        ans[i] = a[i] - sub;
    }

    for i in 0..n {
        if ans[i] > 0 && cnt < k {
            cnt += 1;
            ans[i] -= 1;
        }
    }

    println!("{}", ans.iter().join(" "));
}
