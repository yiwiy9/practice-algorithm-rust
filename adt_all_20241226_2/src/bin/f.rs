use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut a = vec![];
    let mut r = vec![];
    for _ in 0..m {
        input! {
            c_i: usize,
            a_i: [Usize1; c_i],
            r_i: char,
        }
        a.push(a_i);
        r.push(r_i);
    }

    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut ok = true;
        for i in 0..m {
            let mut cnt = 0;
            for &a_i in &a[i] {
                if bit & (1 << a_i) != 0 {
                    cnt += 1;
                }
            }

            if r[i] == 'o' && cnt < k {
                ok = false;
                break;
            }
            if r[i] == 'x' && cnt >= k {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
