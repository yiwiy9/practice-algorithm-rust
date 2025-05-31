use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }

    let mut left_sums = vec![0; n + 1];
    for i in 0..n {
        left_sums[i + 1] = (left_sums[i] + a[i]).min(l * (i as i64 + 1));
    }

    let mut right_sums = vec![0; n + 1];
    for i in (0..n).rev() {
        right_sums[i] = (right_sums[i + 1] + a[i]).min(r * (n as i64 - i as i64));
    }

    let mut ans = 1 << 60;
    for i in 0..=n {
        ans = ans.min(left_sums[i] + right_sums[i]);
    }

    println!("{}", ans);
}
