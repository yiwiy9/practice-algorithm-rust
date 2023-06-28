use proconio::input;

// https://atcoder.jp/contests/abc186/editorial/402
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    a.sort();
    let mut s = vec![0; n + 1];
    for (i, &a_i) in a.iter().enumerate() {
        s[i + 1] += s[i] + a_i;
    }

    let mut ans = 0;
    for (i, &a_i) in a.iter().enumerate() {
        ans += (s[n] - s[i]) - (n as i64 - i as i64) * a_i;
    }

    println!("{}", ans);
}
