use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = vec![(0, 0); n];
    for i in (1..n).rev() {
        let mut digit = 1;
        while digit <= a[i] {
            digit *= 10;
        }
        b[i - 1] = ((b[i].0 + a[i]) % MOD, (b[i].1 + digit) % MOD);
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        ans += b[i].0;
        ans %= MOD;
        ans += (a[i] * b[i].1) % MOD;
        ans %= MOD;
    }

    println!("{}", ans);
}
