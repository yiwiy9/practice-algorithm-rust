use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n-1],
    }

    a.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    let mut j = 0;
    for i in 0..n {
        if j == n - 1 {
            ans = a[i];
            break;
        }
        if a[i] > b[j] {
            if ans != 0 {
                println!("-1");
                return;
            } else {
                ans = a[i];
            }
        } else {
            j += 1;
        }
    }

    println!("{}", ans);
}
