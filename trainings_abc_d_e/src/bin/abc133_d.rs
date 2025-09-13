use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut cur = 0;
    for &a_i in &a {
        cur = a_i - cur;
    }

    let mut ans = vec![cur];
    let mut x = cur / 2;
    for &a_i in &a[..n - 1] {
        ans.push((a_i - x) * 2);
        x = a_i - x;
    }

    println!("{}", ans.iter().join(" "));
}
