use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
    }

    let mut p = vec![];
    let mut f = vec![];
    for _ in 0..n {
        input! {
            p_i: usize,
            c_i: usize,
            f_i: [usize; c_i],
        }
        p.push(p_i);
        f.push(f_i);
    }

    for i in 0..n {
        for j in 0..n {
            if p[i] < p[j] {
                continue;
            }
            if f[i].iter().any(|&x| !f[j].contains(&x)) {
                continue;
            }

            if p[i] > p[j] || f[i].len() < f[j].len() {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
