use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
    }

    let mut p = vec![0; n];
    let mut f = vec![vec![]; n];
    for i in 0..n {
        input! {
            p_i: usize,
            c_i: usize,
            f_i: [usize; c_i],
        }
        p[i] = p_i;
        f[i] = f_i;
    }

    let mut ans = false;
    for i in 0..n {
        for j in 0..n {
            if p[i] < p[j] {
                continue;
            }

            let mut all_included = true;
            for &f_i in &f[i] {
                if !f[j].contains(&f_i) {
                    all_included = false;
                    break;
                }
            }

            if !all_included {
                continue;
            }

            if p[i] > p[j] || f[i].len() < f[j].len() {
                ans = true;
                break;
            }
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
