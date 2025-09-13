use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut left = vec![0; n];
    left[0] = 1;
    for i in 1..n {
        if a[i] > left[i - 1] + 1 {
            left[i] = left[i - 1] + 1;
        } else {
            left[i] = a[i];
        }
    }

    let mut right = vec![0; n];
    right[n - 1] = 1;
    for i in (0..n - 1).rev() {
        if a[i] > right[i + 1] + 1 {
            right[i] = right[i + 1] + 1;
        } else {
            right[i] = a[i];
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(left[i].min(right[i]));
    }

    println!("{}", ans);
}
