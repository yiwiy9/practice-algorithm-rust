use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut ans = 0_usize;
    let mut same_cnt = 0;
    for i in 0..n {
        if a[i] == i {
            ans += same_cnt;
            same_cnt += 1;
        } else if a[i] < i && a[a[i]] == i {
            ans += 1;
        }
    }

    println!("{}", ans);
}
