use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut ans = 0;
    for i in 0..=n {
        let mut cnt = 0;
        for j in 0..n {
            if a[j] >= i {
                cnt += 1;
            }
        }
        if cnt >= i {
            ans = ans.max(i);
        }
    }

    println!("{}", ans);
}
