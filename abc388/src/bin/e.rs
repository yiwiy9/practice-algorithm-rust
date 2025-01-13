use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut left = -1;
    let mut right = n as i64;
    while right - left > 1 {
        let mid = ((left + right) / 2) as usize;

        let mut ok = true;
        for i in 0..mid {
            if n - mid <= i {
                ok = false;
                break;
            }
            if a[i] * 2 > a[n - mid + i] {
                ok = false;
                break;
            }
        }

        if ok {
            left = mid as i64;
        } else {
            right = mid as i64;
        }
    }

    println!("{}", left);
}
