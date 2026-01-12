use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut s = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        s[i + 1] = s[i] + a[i % n];
        s[i + 1] %= m;
    }

    let mut mod_cnt = vec![0; m];
    for i in 0..n {
        mod_cnt[s[i]] += 1;
    }

    let mut ans: usize = 0;
    for i in 0..n {
        mod_cnt[s[i]] -= 1;

        ans += mod_cnt[s[i]];

        mod_cnt[s[i + n]] += 1;
    }

    println!("{}", ans);
}
