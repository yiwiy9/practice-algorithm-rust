use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: i64,
        mut a: [i64; n],
    }

    a.sort_by(|a, b| b.cmp(a));

    let mut cnt = 1;
    let mut ans = 0;
    for i in 0..n {
        let next = if i == n - 1 { 0 } else { a[i + 1] };
        if (a[i] - next) * cnt <= k {
            let last = next + 1;
            ans += (a[i] - next) * (a[i] + last) / 2 * cnt;
            k -= (a[i] - next) * cnt;
        } else {
            let last = a[i] - (k / cnt) + 1;
            ans += (k / cnt) * (a[i] + last) / 2 * cnt;
            ans += (last - 1) * (k % cnt);
            break;
        }
        cnt += 1;
    }

    println!("{}", ans);
}
