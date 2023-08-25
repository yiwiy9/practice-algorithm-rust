use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut s_a = vec![0; n + 1];
    a.iter().enumerate().for_each(|(i, a_i)| {
        s_a[i + 1] = a_i + s_a[i];
    });

    let mut s_b = vec![0; m + 1];
    b.iter().enumerate().for_each(|(i, b_i)| {
        s_b[i + 1] = b_i + s_b[i];
    });

    let mut ans = 0;
    for (i, &s_a_i) in s_a.iter().enumerate() {
        if s_a_i > k {
            break;
        }
        let cnt = i + s_b.upper_bound(&(k - s_a_i)) - 1;
        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
