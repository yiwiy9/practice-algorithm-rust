use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [usize; n],
        a: [usize; n],
        b: [usize; n],
    }

    let mut ans = 0;
    for a_cnt in 0..=1_000_000 {
        let mut q_clone = q.clone();

        let mut ok = true;
        for i in 0..n {
            if a[i] * a_cnt <= q_clone[i] {
                q_clone[i] -= a[i] * a_cnt;
            } else {
                ok = false;
                break;
            }
        }

        if !ok {
            continue;
        }

        let mut b_cnt = 1_000_000;
        for i in 0..n {
            if b[i] == 0 {
                continue;
            }
            let b_i_cnt = q_clone[i] / b[i];
            b_cnt = b_cnt.min(b_i_cnt);
        }

        ans = ans.max(a_cnt + b_cnt);
    }

    println!("{}", ans);
}
