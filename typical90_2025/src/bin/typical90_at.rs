use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut a_cnt = vec![0; 46];
    let mut b_cnt = vec![0; 46];
    let mut c_cnt = vec![0; 46];
    for i in 0..n {
        a_cnt[a[i] % 46] += 1;
        b_cnt[b[i] % 46] += 1;
        c_cnt[c[i] % 46] += 1;
    }

    let mut ans: usize = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += a_cnt[i] * b_cnt[j] * c_cnt[k];
                }
            }
        }
    }

    println!("{}", ans);
}
