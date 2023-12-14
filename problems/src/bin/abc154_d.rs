use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    let mut ans = 0.;
    let mut cur = 0.;
    for (i, &p_i) in p.iter().enumerate() {
        cur += (p_i as f64 + 1.) / 2.;

        if i + 1 < k {
            continue;
        } else {
            if ans < cur {
                ans = cur;
            }
            cur -= (p[i + 1 - k] as f64 + 1.) / 2.;
        }
    }

    println!("{}", ans);
}
