use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![vec![n; n]; n];
    ans[0][(n - 1) / 2] = 1;
    let mut before = (0, (n - 1) / 2);
    for _ in 0..n * n - 1 {
        let k = ans[before.0][before.1];
        let mut r = (before.0 + n - 1) % n;
        let mut c = (before.1 + 1) % n;
        if ans[r][c] == n {
            ans[r][c] = k + 1;
        } else {
            r = (before.0 + 1) % n;
            c = before.1;
            ans[r][c] = k + 1;
        }
        before = (r, c);
    }

    println!("{}", ans.iter().map(|row| row.iter().join(" ")).join("\n"));
}
