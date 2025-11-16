use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
    }

    a.sort_by(|a, b| b.cmp(a));

    let a_max_min = x * a[0];
    let mut ok = true;
    for i in 1..n {
        let a_i_max = a[i] * y;
        if a_i_max < a_max_min {
            ok = false;
            break;
        }

        if (a_i_max - a_max_min) % (y - x) != 0 {
            ok = false;
            break;
        }
    }

    if !ok {
        println!("-1");
        return;
    }

    let weight = a[n - 1] * y;
    let mut ans = 0;
    for &a_i in &a {
        let mut left = 0;
        let mut right = a_i + 1;
        while right - left > 1 {
            let mid = (left + right) / 2;

            if mid * y + (a_i - mid) * x <= weight {
                left = mid;
            } else {
                right = mid;
            }
        }
        ans += left;
    }

    println!("{}", ans);
}
