use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut need_suffix = vec![0; n];
    need_suffix[n - 1] = a[n - 1];
    for i in (0..n - 1).rev() {
        need_suffix[i] = a[i].max(need_suffix[i + 1].saturating_sub(b[i]));
    }

    let mut s_b = vec![0; n + 1];
    for i in 0..n {
        s_b[i + 1] = s_b[i] + b[i];
    }

    let mut need_prefix = vec![0; n];
    need_prefix[0] = a[0];
    for i in 1..n {
        need_prefix[i] = need_prefix[i - 1].max(a[i].saturating_sub(s_b[i]));
    }

    let mut ans: usize = 1 << 60;
    for i in 0..n {
        let mut x = need_suffix[i];
        if i > 0 {
            x = x.max(need_prefix[i - 1].saturating_sub(s_b[n] - s_b[i]));
        }
        ans = ans.min(x);
    }

    println!("{}", ans);
}
