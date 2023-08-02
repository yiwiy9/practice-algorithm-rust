use proconio::input;

/**
 * 尺取り法(しゃくとり法)
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut j = 0;
    let mut sum = 0;
    for &a_i in &a {
        while j < n {
            if sum >= k {
                break;
            }
            sum += a[j];
            j += 1;
        }
        if sum >= k {
            ans += n - j + 1;
        }
        sum -= a_i;
    }

    println!("{}", ans);
}
