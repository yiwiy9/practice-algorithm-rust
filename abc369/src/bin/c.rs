use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut ans = n;
    let mut i = 0;
    while i < n - 1 {
        let mut j = i + 1;
        let d = a[j] - a[i];
        ans += 1;
        while j < n - 1 && a[j + 1] - a[j] == d {
            ans += j - i + 1;
            j += 1;
        }
        i = j;
    }

    println!("{}", ans);
}
