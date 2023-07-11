use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize,usize); n],
    }

    let mut left = 0;
    let mut right = 1_000_000_000 + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut cnt = 0;
        for &(a, b) in &ab {
            if a >= mid {
                cnt += b;
            }
        }

        if cnt > k {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", right);
}
