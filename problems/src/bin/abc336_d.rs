use proconio::input;

/**
 * https://atcoder.jp/contests/abc336/tasks/abc336_d
 * https://atcoder.jp/contests/abc336/editorial/9078
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut l = vec![0; n + 2];
    let mut r = vec![0; n + 2];
    for i in 0..n {
        l[i + 1] = a[i].min(l[i] + 1);
    }
    for i in (0..n).rev() {
        r[i + 1] = a[i].min(r[i + 2] + 1);
    }

    println!("{}", (0..n).map(|i| l[i + 1].min(r[i + 1])).max().unwrap());
}
