use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut ans = 0;
    let mut cnt = 0;
    let mut j = 0;
    for &a_i in &a {
        while j < n && a[j] < a_i + m {
            j += 1;
            cnt += 1;
        }
        ans = ans.max(cnt);
        cnt -= 1;
    }

    println!("{}", ans);
}
