use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }

    a.sort();

    let mut ans = a[n - 1] - a[k];
    for i in 0..=k {
        ans = ans.min(a[n - 1 - i] - a[k - i]);
    }

    println!("{}", ans);
}
