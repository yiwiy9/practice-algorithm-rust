use proconio::input;

fn main() {
    input! {
        n: usize,
        m: isize,
        mut a: [isize; n],
    }

    a.sort();

    let mut left = -1;
    let mut right = 1_000_000_000 + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut sum = 0;
        for &a_i in &a {
            sum += a_i.min(mid);
        }

        if sum <= m {
            left = mid;
        } else {
            right = mid;
        }
    }

    if left == 1_000_000_000 {
        println!("infinite");
        return;
    }

    println!("{}", left);
}
