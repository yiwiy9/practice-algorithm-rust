use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut left = 0;
    let mut last_x = n;
    let mut last_y = n;
    for right in 0..n {
        if a[right] > x || a[right] < y {
            left = right + 1;
            last_x = n;
            last_y = n;
            continue;
        }

        if a[right] == x {
            last_x = right;
        }
        if a[right] == y {
            last_y = right;
        }

        if last_x <= right && last_y <= right {
            ans += last_x.min(last_y) + 1 - left;
        }
    }

    println!("{}", ans);
}
