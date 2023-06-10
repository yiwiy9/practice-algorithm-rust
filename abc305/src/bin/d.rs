use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize,usize); q],
    }

    let mut s = vec![0; n];
    for i in 1..n {
        if i % 2 != 0 {
            s[i] = s[i - 1]
        } else {
            s[i] = s[i - 1] + a[i] - a[i - 1]
        }
    }

    for (l, r) in &lr {
        let l_upper = a.upper_bound(l);
        let r_upper = a.upper_bound(r);

        if l_upper == r_upper {
            if l_upper % 2 != 0 {
                println!("{}", 0);
            } else {
                println!("{}", r - l);
            }
            continue;
        }

        let mut ans = 0;
        if l_upper % 2 == 0 {
            ans += a[l_upper] - l;
        }
        if (r_upper - 1) % 2 != 0 {
            ans += r - a[r_upper - 1];
        }
        ans += s[r_upper - 1] - s[l_upper];

        println!("{}", ans);
    }
}
