use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut left = vec![0; n];
    for i in 0..n - 1 {
        left[i + 1] = left[i]
            + if a[i] >= a[i + 1] {
                a[i] - a[i + 1] + 1
            } else {
                0
            };
    }

    let mut right = vec![0; n + 1];
    for i in (0..n - 1).rev() {
        right[i] = right[i + 1]
            + if a[i + 1] >= a[i] {
                a[i + 1] - a[i] + 1
            } else {
                0
            };
    }

    let mut ans: usize = 1 << 60;
    for i in 0..n {
        ans = ans.min(left[i].max(right[i]));
    }

    println!("{}", ans);
}
