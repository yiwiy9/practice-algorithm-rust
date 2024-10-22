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
    }

    let s_mod_m = s.iter().map(|&x| x % m).collect::<Vec<_>>();

    let mut mod_cnt = vec![0; m];
    for i in 0..n {
        mod_cnt[s_mod_m[i]] += 1;
    }

    let mut ans = 0_usize;
    for i in 0..n {
        mod_cnt[s_mod_m[i]] -= 1;
        ans += mod_cnt[s_mod_m[i]];
        mod_cnt[s_mod_m[i + n]] += 1;
    }

    println!("{}", ans);
}
