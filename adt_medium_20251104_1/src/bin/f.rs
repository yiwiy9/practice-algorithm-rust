use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut ans = a.len();
    let mut cnt = 1;
    for i in 1..n - 1 {
        if a[i] - a[i - 1] == a[i + 1] - a[i] {
            cnt += 1;
        } else {
            ans += cnt * (cnt + 1) / 2;
            cnt = 1;
        }
    }
    if n > 1 {
        ans += cnt * (cnt + 1) / 2;
    }

    println!("{}", ans);
}
