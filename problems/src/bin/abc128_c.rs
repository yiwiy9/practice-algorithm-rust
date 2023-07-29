use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut s_vec = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            s: [Usize1; k],
        }
        s_vec.push(s);
    }
    input! {
        p: [usize; m],
    }

    let mut ans = 0;
    for bit in 0..(1 << n) {
        let mut ok = true;
        for i in 0..m {
            let mut cnt = 0;
            for &s_i in &s_vec[i] {
                if bit & (1 << s_i) != 0 {
                    cnt += 1;
                }
            }
            if cnt % 2 != p[i] {
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
