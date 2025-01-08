use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut left = -1;
    let mut right = 1 << 60;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut ok = true;
        let mut cur = mid;
        for &a_i in &a {
            cur += a_i;
            if cur < 0 {
                ok = false;
                break;
            }
        }

        if ok {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", a.iter().sum::<i64>() + right);
}
