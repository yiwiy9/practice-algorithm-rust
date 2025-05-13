use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = a[i] + s[i];
    }

    let mut ans = 0;
    for i in 0..n {
        ans += a[i] * (s[n] - s[i + 1]);
    }

    println!("{}", ans);
}
