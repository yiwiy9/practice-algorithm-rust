use proconio::input;

/**
 * https://atcoder.jp/contests/abc367/tasks/abc367_d
 * https://atcoder.jp/contests/abc367/editorial/10706
 *
 * Ri　= ( 休憩所 0 から出発して、 i 番目の休憩所として休憩所 i%N に到着するまでにかかる歩数を M で割った余り ) とします。
 * これを i が 0 から 2N−1 まで計算しておきます。
 *
 * R0 から RN−1 において 0 から M−1 までの k の出現頻度を 記録した配列を B=(B[0],B[1],…,B[M−1]) とし、
 * i = N ~ 2N−1 の繰り返しで、常に長さ N の区間において B の出現頻度を更新していきます。（尺取り法的なアプローチ）
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut r = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        r[i + 1] = (r[i] + a[i % n]) % m;
    }

    let mut b = vec![0; m];
    for i in 0..n {
        b[r[i]] += 1;
    }

    let mut ans: usize = 0;
    // t(=i)を固定して考える
    for i in n..2 * n {
        b[r[i - n]] -= 1;
        ans += b[r[i]];
        b[r[i]] += 1;
    }

    println!("{}", ans);
}
