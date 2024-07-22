use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        l: [usize; n],
    }

    let mut ans = 0;
    loop {
        let mut cnt = 0;
        for &l_i in &l {
            if l_i + ans >= t {
                cnt += 1;
            }
        }
        if p <= cnt {
            break;
        }
        ans += 1;
    }

    println!("{}", ans);
}
