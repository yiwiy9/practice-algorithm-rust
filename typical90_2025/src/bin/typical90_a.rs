use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.push(l);

    let mut left = 0;
    let mut right = l + 1;
    while right - left > 1 {
        let mid = (right + left) / 2;

        let mut cnt = 0;
        let mut before = 0;
        for &a_i in &a {
            if a_i - before >= mid {
                cnt += 1;
                before = a_i;
            }
        }

        if cnt >= k + 1 {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
