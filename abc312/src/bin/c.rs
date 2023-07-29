use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut left = 0;
    let mut right = 1_000_000_000 + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut a_cnt = 0;
        for &a_i in &a {
            if mid >= a_i {
                a_cnt += 1;
            }
        }

        let mut b_cnt = 0;
        for &b_i in &b {
            if mid <= b_i {
                b_cnt += 1;
            }
        }

        if a_cnt >= b_cnt {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
