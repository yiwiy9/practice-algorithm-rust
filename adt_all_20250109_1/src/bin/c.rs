use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut cnt = 0;
    let mut cur = 0;
    let mut i = 0;
    while i < n {
        if cur + a[i] > k {
            cur = 0;
            cnt += 1;
            continue;
        }
        cur += a[i];
        i += 1;
    }
    if cur > 0 {
        cnt += 1;
    }

    println!("{}", cnt);
}
